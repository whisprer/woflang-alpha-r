//! Fractal mathematics operations.
//!
//! ## Operations
//!
//! - `mandelbrot` - Escape-time iteration count for a complex point
//! - `sierpinski` - ASCII Sierpinski triangle
//! - `hausdorff_dim` - Self-similar Hausdorff dimension
//! - `julia` - Julia set iteration count

use woflang_core::{InterpreterContext, WofValue};
use woflang_runtime::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════
// FRACTAL ALGORITHMS
// ═══════════════════════════════════════════════════════════════════════════

/// Compute Mandelbrot escape iterations for c = cr + i*ci.
fn mandelbrot_escape(cr: f64, ci: f64, max_iter: i32) -> i32 {
    let mut zr = 0.0;
    let mut zi = 0.0;
    let mut iter = 0;

    while iter < max_iter {
        let zr2 = zr * zr;
        let zi2 = zi * zi;

        if zr2 + zi2 > 4.0 {
            break;
        }

        let new_zr = zr2 - zi2 + cr;
        let new_zi = 2.0 * zr * zi + ci;

        zr = new_zr;
        zi = new_zi;
        iter += 1;
    }

    iter
}

/// Compute Julia set escape iterations for z starting at zr + i*zi.
fn julia_escape(zr: f64, zi: f64, cr: f64, ci: f64, max_iter: i32) -> i32 {
    let mut zr = zr;
    let mut zi = zi;
    let mut iter = 0;

    while iter < max_iter {
        let zr2 = zr * zr;
        let zi2 = zi * zi;

        if zr2 + zi2 > 4.0 {
            break;
        }

        let new_zr = zr2 - zi2 + cr;
        let new_zi = 2.0 * zr * zi + ci;

        zr = new_zr;
        zi = new_zi;
        iter += 1;
    }

    iter
}

/// Print ASCII Sierpinski triangle.
fn print_sierpinski(depth: i32) {
    let depth = depth.clamp(1, 8);
    let size = 1 << depth;

    println!("[fractal] Sierpinski triangle (depth {})", depth);

    for y in 0..size {
        // Centering
        for _ in 0..(size - y) {
            print!(" ");
        }

        for x in 0..size {
            if (x & y) == 0 {
                print!("*");
            } else {
                print!(" ");
            }
        }

        println!();
    }
}

/// Hausdorff/self-similar dimension: D = log(N) / log(scale).
fn hausdorff_dimension(n: f64, scale: f64) -> f64 {
    if n <= 0.0 || scale <= 0.0 || (scale - 1.0).abs() < f64::EPSILON {
        return f64::NAN;
    }
    n.ln() / scale.ln()
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register all fractal operations.
pub fn register(interp: &mut Interpreter) {
    // Mandelbrot: real imag max_iter → iterations
    interp.register("mandelbrot", |interp| {
        let max_iter = interp.stack_mut().pop()?.as_int()?.clamp(1, 10000) as i32;
        let imag = interp.stack_mut().pop()?.as_float()?;
        let real = interp.stack_mut().pop()?.as_float()?;

        let iters = mandelbrot_escape(real, imag, max_iter);

        let status = if iters == max_iter {
            "(likely in set)"
        } else {
            "(escaped)"
        };
        println!(
            "[fractal] mandelbrot({} + {}i, max_iter={}) → iters={} {}",
            real, imag, max_iter, iters, status
        );

        interp.stack_mut().push(WofValue::integer(iters as i64));
        Ok(())
    });

    // Julia: zr zi cr ci max_iter → iterations
    interp.register("julia", |interp| {
        let max_iter = interp.stack_mut().pop()?.as_int()?.clamp(1, 10000) as i32;
        let ci = interp.stack_mut().pop()?.as_float()?;
        let cr = interp.stack_mut().pop()?.as_float()?;
        let zi = interp.stack_mut().pop()?.as_float()?;
        let zr = interp.stack_mut().pop()?.as_float()?;

        let iters = julia_escape(zr, zi, cr, ci, max_iter);

        let status = if iters == max_iter {
            "(likely in set)"
        } else {
            "(escaped)"
        };
        println!(
            "[fractal] julia(z={} + {}i, c={} + {}i, max={}) → {} {}",
            zr, zi, cr, ci, max_iter, iters, status
        );

        interp.stack_mut().push(WofValue::integer(iters as i64));
        Ok(())
    });

    // Sierpinski: depth → ()
    interp.register("sierpinski", |interp| {
        let depth = interp.stack_mut().pop()?.as_int()? as i32;
        print_sierpinski(depth);
        Ok(())
    });

    // Hausdorff dimension: N scale → dimension
    // D = log(N) / log(scale)
    interp.register("hausdorff_dim", |interp| {
        let scale = interp.stack_mut().pop()?.as_float()?;
        let n = interp.stack_mut().pop()?.as_float()?;

        let d = hausdorff_dimension(n, scale);

        println!("[fractal] hausdorff_dim(N={}, scale={}) = {}", n, scale, d);

        interp.stack_mut().push(WofValue::Float(d));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // WELL-KNOWN DIMENSIONS
    // ─────────────────────────────────────────────────────────────────────

    // Sierpinski triangle dimension: log(3)/log(2) ≈ 1.585
    interp.register("dim_sierpinski", |interp| {
        let d = 3.0_f64.ln() / 2.0_f64.ln();
        interp.stack_mut().push(WofValue::Float(d));
        Ok(())
    });

    // Koch curve dimension: log(4)/log(3) ≈ 1.262
    interp.register("dim_koch", |interp| {
        let d = 4.0_f64.ln() / 3.0_f64.ln();
        interp.stack_mut().push(WofValue::Float(d));
        Ok(())
    });

    // Cantor set dimension: log(2)/log(3) ≈ 0.631
    interp.register("dim_cantor", |interp| {
        let d = 2.0_f64.ln() / 3.0_f64.ln();
        interp.stack_mut().push(WofValue::Float(d));
        Ok(())
    });

    // Menger sponge dimension: log(20)/log(3) ≈ 2.727
    interp.register("dim_menger", |interp| {
        let d = 20.0_f64.ln() / 3.0_f64.ln();
        interp.stack_mut().push(WofValue::Float(d));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // ASCII ART FRACTALS
    // ─────────────────────────────────────────────────────────────────────

    // Simple mandelbrot ASCII visualization
    // Stack: x_min x_max y_min y_max width height max_iter → ()
    interp.register("mandelbrot_ascii", |interp| {
        let max_iter = interp.stack_mut().pop()?.as_int()?.clamp(1, 100) as i32;
        let height = interp.stack_mut().pop()?.as_int()?.clamp(1, 50) as i32;
        let width = interp.stack_mut().pop()?.as_int()?.clamp(1, 100) as i32;
        let y_max = interp.stack_mut().pop()?.as_float()?;
        let y_min = interp.stack_mut().pop()?.as_float()?;
        let x_max = interp.stack_mut().pop()?.as_float()?;
        let x_min = interp.stack_mut().pop()?.as_float()?;

        let chars = " .:-=+*#%@";
        let char_vec: Vec<char> = chars.chars().collect();

        for py in 0..height {
            for px in 0..width {
                let x = x_min + (x_max - x_min) * (px as f64) / (width as f64);
                let y = y_min + (y_max - y_min) * (py as f64) / (height as f64);

                let iters = mandelbrot_escape(x, y, max_iter);
                let idx = if iters == max_iter {
                    char_vec.len() - 1
                } else {
                    (iters as usize * char_vec.len() / max_iter as usize).min(char_vec.len() - 1)
                };

                print!("{}", char_vec[idx]);
            }
            println!();
        }

        Ok(())
    });

    // Help
    interp.register("fractal_help", |_interp| {
        println!("Fractal Operations:");
        println!();
        println!("  Iteration counts:");
        println!("    real imag max_iter mandelbrot → iterations");
        println!("    zr zi cr ci max_iter julia    → iterations");
        println!();
        println!("  Visualization:");
        println!("    depth sierpinski              → (prints triangle)");
        println!("    x1 x2 y1 y2 w h max mandelbrot_ascii → (prints fractal)");
        println!();
        println!("  Dimensions:");
        println!("    N scale hausdorff_dim         → log(N)/log(scale)");
        println!("    dim_sierpinski                → ~1.585");
        println!("    dim_koch                      → ~1.262");
        println!("    dim_cantor                    → ~0.631");
        println!("    dim_menger                    → ~2.727");
        Ok(())
    });
}
