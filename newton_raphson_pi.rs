use std::f64::consts::PI;

fn newton_raphson_pi() -> f64 {
    // Parameter settings
    let tolerance = 1e-15;       // Convergence tolerance
    let max_iterations = 50;     // Maximum number of iterations
    let mut x: f64 = 3.14;       // Initial value (close to π) - explicitly specify f64
    let mut prev_x: f64;
    let mut iteration = 0;

    println!("Iterations\tx value\t\t\tError\t\t\tFunction value");
    println!("---------------------------------------------------------------");

    loop {
        prev_x = x;
        
        // Newton-Raphson iteration: x_{n+1} = x_n - tan(x_n)
        x = x - x.tan();
        
        iteration += 1;
        
        // Calculate the current error and function value
        let error = (x - prev_x).abs();
        let function_value = x.sin().abs();
        
        println!("{:2}\t\t{:.16}\t{:.4e}\t{:.4e}", iteration, x, error, function_value);
        
        // Check convergence conditions
        if error < tolerance || function_value < tolerance {
            println!("converges after {} iterations", iteration);
            break;
        }
        
        // Check the maximum number of iterations
        if iteration >= max_iterations {
            println!("reached the maximum number of iterations {}", max_iterations);
            break;
        }
    }
    
    x
}

fn main() {
    let pi_approx = newton_raphson_pi();
    let pi_exact = PI;
    let absolute_error = (pi_approx - pi_exact).abs();
    
    println!("\nCalculation result:");
    println!("Approximate value π = {:.16}", pi_approx);
    println!("Exact value π = {:.16}", pi_exact);
    println!("Absolute error = {:.4e}", absolute_error);
    println!("Relative error = {:.4e}", absolute_error / pi_exact);
}
