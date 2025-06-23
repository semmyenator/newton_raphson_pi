
# Using the Newton-Raphson Method to Calculate Pi (π)
This project demonstrates how to use the Newton-Raphson numerical method to solve the equation sin(x) = 0, thereby calculating an approximate value of π.
The Newton-Raphson method is an efficient iterative technique that rapidly converges to the root of an equation using information from the function and its derivative.

Mathematical Principles
Newton-Raphson Method
The Newton-Raphson method is an iterative root-finding algorithm with the basic formula:
x_{n+1} = x_n - f(x_n) / f'(x_n)

Where:
x_n is the approximate value at the nth iteration
f(x) is the equation for which the root is to be found
f'(x) is the derivative of the function

Solving sin(x) = 0
We choose the equation sin(x) = 0 because π is one of the positive roots of this equation (sin(π) = 0).

Define the function and its derivative:
f(x) = sin(x)
f'(x) = cos(x)

Iteration Formula:
Plugging into the Newton-Raphson formula:
x_{n+1} = x_n - sin(x_n) / cos(x_n) = x_n - tan(x_n)

Choosing the Initial Value:
Select a value close to π as the initial value, such as x₀ = 3.14 (since π ≈ 3.14).

Convergence Condition:
Stop iterating when |x_{n+1} - x_n| < ε or |sin(x_n)| < ε (where ε is a predefined tolerance).

Why Can We Calculate π?
Although sin(x) = 0 has multiple solutions (x = kπ, where k is an integer), by choosing the initial value x₀ = 3.0 (close to π but away from 0 and 2π), we can ensure that the algorithm converges to π rather than other roots.

Algorithm Characteristics
Quadratic Convergence:
Each iteration roughly doubles the correct digits.
Efficiency:
Typically requires only a few iterations to reach machine precision.
Dependence on Initial Value:
The initial value must be close to the target root (π) for convergence.
Numerical Stability:
Stable near π, since cos(π) = -1 ≠ 0.

Detailed Mathematical Derivation
Geometric Explanation of the Newton-Raphson Method
At the point (xₙ, f(xₙ)), draw a tangent line; the intersection of the tangent with the x-axis is the next approximate value xₙ₊₁.
The iteration process is as follows:
xₙ₊₁ = xₙ - f(xₙ) / f'(xₙ)

Convergence Analysis
Near π, the algorithm satisfies the convergence conditions of the Newton-Raphson method:
f(π) = sin(π) = 0
f'(π) = cos(π) = -1 ≠ 0
The second derivative f''(x) = -sin(x) is bounded near π.
The convergence rate is quadratic:
|eₙ₊₁| ≈ |f''(ξ) / (2f'(xₙ))||eₙ|²
where eₙ = xₙ - π is the error at the nth step.

Notes
Choosing the Initial Value:
The initial value should be close to π (3.14) to avoid converging to other roots.
Numerical Stability:
A division by zero error may occur when cos(xₙ) is close to zero (add protective measures).
Termination Condition:
Set a maximum number of iterations to prevent infinite loops.
Precision Limitations:
Limited by floating-point precision (IEEE754 double precision is about 16 decimal places).
