use crate::types::*;
/* 
    <FIRST READ THE COMMENT IN impulse_response.rs>
    In the windowing method, we multiply, element-wise, the desired impulse response (highpass, lowpass, etc.) 
    with a window function. Window functions can be of different types such as rectangular, Hann, Hamming, etc. 
    However, a common property of all of them is that they are symmetric.
    As a consequence, when they multiply the desired impulse response, the result is either symmetric or antisymmetric.
    Hence, the resulting function can be thought of as FIR filter taps which gives us linear phase.

    Now, let's see what happens in the frequency domain when we multiply a delayed symmetric impulse response with a window function.
    Assume the desired impulse response is symmetric and has length \(M+1\).
    Since it is symmetric about \(M/2\), its DTFT will be
    
    \[D(e^{j\omega}) = A(e^{j\omega}) e^{-j\omega M/2}\]

    where \(A(e^{j\omega})\) is a real and even function (because of the symmetry),
    and \(e^{-j\omega M/2}\) denotes a delay of \(M/2\) samples in time.

    Assume the window function is rectangular. Since it is also symmetric, the DTFT of this function is
    
    \[
    W(e^{j\omega}) = B(e^{j\omega}) e^{-j\omega M/2}
    \]

    where \(B(e^{j\omega})\) is also real and even.

    Since the window function multiplies the impulse response in time, in the frequency domain it becomes periodic convolution:

    \[
    H(e^{j\omega}) = \frac{1}{2\pi} \int_{-\pi}^{\pi} W(e^{j\theta}) D(e^{j(\omega-\theta)}) \, d\theta
    \]

    Plugging the two functions into the definition of periodic convolution gives:

    \[
    H(e^{j\omega}) = e^{-j\omega M/2} \frac{1}{2\pi} \int_{-\pi}^{\pi} B(e^{j\theta}) A(e^{j(\omega-\theta)}) \, d\theta
    \]

    (Because in the convolution, \(e^{-j\theta M/2} e^{-j(\omega - \theta) M/2} = e^{-j\omega M/2}\)).

    The integral is a periodic convolution of two real and even functions, so the result is also real and even. Hence,

    \[
    H(e^{j\omega}) = C(e^{j\omega}) e^{-j\omega M/2}
    \]

    where \(C(e^{j\omega})\) is real and even, and \(e^{-j\omega M/2}\) denotes a delay of \(M/2\) samples in time.
*/
use crate::filter_design::impulse_response::get_impulse_response;
use crate::filter_design::window;

pub fn windowing_method(n: usize, window_type: WindowType, filter_type: FilterType) -> TransferFunction
{
    match window_type {
        WindowType::Rectangular => {
            get_fir_tf(get_impulse_response(n, filter_type).to_vec())
        }
        WindowType::Bartlett => {
             get_fir_tf((get_impulse_response(n, filter_type) * window::bartlett(n)).to_vec())
        }
        WindowType::Hamming => {
             get_fir_tf((get_impulse_response(n, filter_type) * window::hamming(n)).to_vec())
        }
        WindowType::Han => {
             get_fir_tf((get_impulse_response(n, filter_type) * window::han(n)).to_vec())
        }
        WindowType::Kaiser { min_stopband_attinuation, transition_width } => {
            let window = window::kaiser(min_stopband_attinuation, transition_width);
            get_fir_tf((get_impulse_response(window.len(), filter_type) * window).to_vec())
        }
    }

}

fn get_fir_tf(a_ks: Vec<f64>) -> TransferFunction {
    TransferFunction {
        num: a_ks,
        den: vec![1.0],
    }
}