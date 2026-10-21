use crate::signal::Signal;
use num_complex::Complex;
use std::f64::consts::PI;

type Complex64 = Complex<f64>;

impl Signal {
    pub fn fft(&self) -> Vec<Complex64> {
        let mut signal = self.clone();
        signal.zero_pad_to_the_next_power_of_two();

        let input: Vec<Complex64> = signal
            .data
            .iter()
            .map(|&x| Complex64::new(x, 0.0))
            .collect();

        fft_recursive(&input)
    }

    pub fn ifft(spectrum: &[Complex64]) -> Signal {
        let n = spectrum.len();

        // Conjugate, compute FFT, conjugate again, and scale
        let conjugated: Vec<Complex64> = spectrum.iter().map(|x| x.conj()).collect();
        let mut result = fft_recursive(&conjugated);

        for x in &mut result {
            *x = x.conj() / n as f64;
        }

        // Convert back to Signal (real values)
        let real_data: Vec<f64> = result.iter().map(|x| x.re).collect();
        Signal::new(real_data)
    }
}
fn get_even_elements(input: &[Complex64]) -> Vec<Complex64> {
    let mut result = Vec::with_capacity(input.len() / 2);
    for i in (0..input.len()).step_by(2) {
        result.push(input[i]);
    }
    result
}

fn get_odd_elements(input: &[Complex64]) -> Vec<Complex64> {
    let mut result = Vec::with_capacity(input.len() / 2);
    for i in (1..input.len()).step_by(2) {
        result.push(input[i]);
    }
    result
}

fn fft_recursive(input: &[Complex64]) -> Vec<Complex64> {
    let n = input.len();
    assert!(
        n.is_power_of_two(),
        "Input length must be a power of 2, got {}",
        n
    );

    if n == 1 {
        return vec![input[0]];
    }

    // Split into even and odd elements
    let even_elements = get_even_elements(input);
    let odd_elements = get_odd_elements(input);

    // Recursive FFT on even and odd parts
    let even_fft = fft_recursive(&even_elements);
    let odd_fft = fft_recursive(&odd_elements);

    // Combine results
    let mut result = vec![Complex64::new(0.0, 0.0); n];

    for k in 0..n / 2 {
        let angle = -2.0 * PI * k as f64 / n as f64;
        let twiddle = Complex64::new(angle.cos(), angle.sin());

        let t = twiddle * odd_fft[k];
        result[k] = even_fft[k] + t;
        result[k + n / 2] = even_fft[k] - t;
    }

    result
}

/*
<################ SOME COMMENTS ON DTFT ########################>
<###############################################################>
FFT is great for computing the DFT, which shows us the frequency content of a signal.
However, the DFT has a big limitation.
The coefficients computed by the DFT depend on the size of the signal.
Therefore, we cannot compare the DFT coefficients of two signals that have different sizes.
To overcome this, we can zero-pad both signals to have the same length.

For analytical purposes, one interesting choice for the number of zeros to pad is infinity!
This gives us the Discrete-time Fourier Transform (DTFT) of a signal.
The DTFT gives us a tool to compare the frequency content of a finite-length signal (which becomes infinite-length after appending infinitely many zeros) with that of signals that genuinely have infinite length.

To compute the DTFT of a signal that is non-zero only from index $N_{1}$ to $N_{2}$, we do the following:

Choose a period $N \geq N_{2} - N_{1} + 1$, construct a periodic extension $\tilde{x}[n]$ of $x[n]$, and apply the DFS.
The DFS coefficients are computed as ($\omega_{0} = 2\pi / N$):

$$
C_k = \frac{1}{N}\sum_{n=N_1}^{N_1+N-1}x[n]e^{-jk\omega_0n}
= \frac{1}{N}X_{N}(e^{jk\omega_0}),
\qquad
X_{N}(e^{j\omega}) \triangleq \sum_{n=N_1}^{N_1+N-1}x[n]e^{-j\omega n}
$$
Now increase $N$. The window $N_1 \leq n \leq N_1+N-1$ grows, but $x[n]=0$ outside $N_1 \leq n \leq N_2$, so the extra terms are all zero and $X_N$ no longer changes. We define this common value as $X(e^{j\omega})$:

$$
X_{N}(e^{j\omega}) = \sum_{n=N_1}^{N_2}x[n]e^{-j\omega n}
= \sum_{n=-\infty}^{\infty}x[n]e^{-j\omega n}
\triangleq X(e^{j\omega})
\quad \text{for every } N \geq N_2-N_1+1
$$

The function $X(e^{j\omega})$ is the Discrete-time Fourier Transform (DTFT). The DFS coefficients $C_k = \frac{1}{N}X(e^{jk\omega_0})$ are scaled samples of it at $N$ equally spaced frequencies $\omega_k = k\omega_0 = \frac{2\pi k}{N}$. Increasing $N$ (zero-padding) makes the samples denser, so $\omega_0 \to 0$.

Substituting $\frac{1}{N}X_{N}(e^{jk\omega_0})$ into the DFS synthesis equation, and using $\frac{1}{N} = \frac{\omega_0}{2\pi}$, we get:


$$
\tilde{x}[n] = \sum_{k=0}^{N-1} C_k e^{jk\omega_0 n}
= \frac{1}{2\pi} \sum_{k=0}^{N-1} X_{N}(e^{jk\omega_0}) e^{jk\omega_0n}\, \omega_0
$$

As $N\rightarrow\infty$, we have $\omega_0\rightarrow0$ and $\tilde{x}[n]\rightarrow x[n]$. Since $X_N = X$ for every $N \geq N_2-N_1+1$, the sum is a Riemann sum of the fixed continuous function $X(e^{j\omega})e^{j\omega n}$ with step $\omega_0$, so it becomes an integral:


$$x[n] = \frac{1}{2\pi} \int_{0}^{2\pi} X(e^{j\omega}) e^{j\omega n}\, d\omega$$

To summarize, the analysis and synthesis equations for the DTFT are:


$$
\boxed{X(e^{j\omega}) = \sum_{n=-\infty}^{\infty}x[n]e^{-j\omega n}}
\qquad
\boxed{x[n] = \frac{1}{2\pi} \int_{0}^{2\pi} X(e^{j\omega}) e^{j\omega n}\, d\omega}
$$

Important: For finite-length $x[n]$, the Riemann sum argument above shows that the DFS synthesis sum becomes the synthesis integral, so the synthesis equation recovers $x[n]$ from $X(e^{j\omega})$. This step was valid because $X_N(e^{j\omega})$ equals $X(e^{j\omega})$ for all sufficiently large $N$, so the sum is a Riemann sum of one fixed function.

Note, however, that the DTFT can also perfectly decompose signals that have infinite length.
Specifically, let $x_M[n]$ equal $x[n]$ for $|n| \leq M$ and zero elsewhere, and let $X_M(e^{j\omega}) = \sum_{m=-M}^{M}x[m]e^{-j\omega m}$ be its DTFT. Since $x_M[n]$ has finite length, the finite-length result above applies to it: the synthesis equation applied to $X_M$ gives back $x_M[n]$, which equals $x[n]$ for every $M \geq |n|$. Hence
$$
\lim_{M\to\infty}\frac{1}{2\pi}\int_{0}^{2\pi}X_M(e^{j\omega})e^{j\omega n}\,d\omega = x[n]
$$

For the synthesis equation to hold for the infinite-length signal, we need this to equal the synthesis equation applied to $X(e^{j\omega})$. That is, we need to be able to move the limit inside the integral:


$$
\lim_{N\to\infty}\frac{1}{2\pi}\int_{0}^{2\pi}X_N(e^{j\omega})e^{j\omega n}\,d\omega
= \frac{1}{2\pi}\int_{0}^{2\pi}\lim_{N\to\infty}X_N(e^{j\omega})e^{j\omega n}\,d\omega
= \frac{1}{2\pi}\int_{0}^{2\pi}X(e^{j\omega})e^{j\omega n}\,d\omega
$$

For the limit to move inside the integral, we need the signal to have special properties.
A sufficient condition is for the signal to be *absolutely summable*, meaning:
$$
\sum_{n=-\infty}^{\infty}|x[n]| < \infty
$$

When this is the case, $X(e^{j\omega})$ converges for every $\omega$, and
$$
\left|X(e^{j\omega}) - X_N(e^{j\omega})\right|
= \left|\sum_{|m|>N}x[m]e^{-j\omega m}\right|
\leq \sum_{|m|>N}|x[m]| \triangleq \varepsilon_N
$$

Since $\varepsilon_N$ does not depend on $\omega$ and $\varepsilon_N \to 0$ as $N\to\infty$ (the tail of a convergent series), the convergence is uniform.
The consequence is that
$$
\left|\frac{1}{2\pi}\int_{0}^{2\pi}X(e^{j\omega})e^{j\omega n}\,d\omega - \frac{1}{2\pi}\int_{0}^{2\pi}X_N(e^{j\omega})e^{j\omega n}\,d\omega\right|
\leq \frac{1}{2\pi}\int_{0}^{2\pi}\varepsilon_N\,d\omega = \varepsilon_N
$$
also approaches zero as $N$ increases. Hence the limit can be moved inside the integral, and the synthesis equation applied to $X(e^{j\omega})$ gives back $x[n]$.

A classic and important example of such a signal is:
$$
x[n] = \left(\tfrac{1}{2}\right)^n u[n]
$$

Using the geometric sum, this signal satisfies
$$
\sum_{n=0}^{\infty}\left(\tfrac{1}{2}\right)^n = \frac{1}{1-\frac{1}{2}} = 2 < \infty
$$

which shows that it is absolutely summable, so we can correctly compute its DTFT,
$$
X(e^{j\omega}) = \sum_{n=0}^{\infty}\left(\tfrac{1}{2}e^{-j\omega}\right)^n = \frac{1}{1-\frac{1}{2}e^{-j\omega}}
$$

Valid since $\left|\tfrac{1}{2}e^{-j\omega}\right| = \tfrac{1}{2} < 1$, and then reconstruct $x[n]$ using the synthesis equation.

*/
