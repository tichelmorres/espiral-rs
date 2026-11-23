use raylib::prelude::*;

const WINDOW_WIDTH			            : i32   = 1280;
const WINDOW_HEIGHT		            	: i32   = 720;

const BASE_SQUARE_SIZE	          		: f32   = 8.0;
const MIN_SQUARE_DISPLAY_SIZE_OPTIMIZED	: f32   = 1.0;
const MIN_SQUARE_DISPLAY_SIZE	     	: f32   = 4.0;
const MAX_SQUARE_DISPLAY_SIZE	    	: f32   = 200.0;

const SIEVE_LIMIT		             	: usize = 841_500;
const INITIAL_VISIBLE_PRIMES	    	: usize = 150;
const PRIMES_INCREMENT		        	: usize = 5;

const INITIAL_ZOOM_LEVEL          		: f64   = 0.67;
const MIN_ZOOM		              		: f64   = 0.000056;
const MAX_ZOOM	             			: f64   = 1000.0;

const ROTATION_SPEED		        	: f64   = 0.0004;
const INITIAL_ROTATION_OFFSET	    	: f64   = 0.0;

fn bool_pt(boolean: bool) -> &'static str {
    if boolean { "sim" } else { "não" }
}

fn theme_name_pt(theme_name: &'static str) -> &'static str {
    if theme_name == "Default" { "Padrão" } else { theme_name }
}

pub fn sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }

    let m = (n - 1) / 2;
    let words = (m + 63) / 64;
    let mut bits = vec![0u64; words];

    let sqrt_n = (n as f64).sqrt() as usize;
    let limit_idx = if sqrt_n >= 3 {
        let idx = (sqrt_n - 3) / 2;
        idx.min(m.saturating_sub(1))
    } else {
        0usize
    };

    for i in 0..=limit_idx {
        let wi = i >> 6;
        let bi = i & 63;

        if (bits[wi] >> bi) & 1 == 0 {
            let p = 2 * i + 3;

            let mut j = (p * p - 3) / 2;
            if j >= m {
                continue;
            }

            let mut wj = j >> 6;
            let mut bj = j & 63;
            let step = p;

            while j < m {
                bits[wj] |= 1u64 << bj;

                let sum = bj + step;
                wj += sum >> 6;
                bj = sum & 63;

                j += step;
            }
        }
    }

    let mut primes = Vec::with_capacity((n / 10).max(4));
    primes.push(2usize);
    for i in 0..m {
        let w = i >> 6;
        let b = i & 63;
        if (bits[w] >> b) & 1 == 0 {
            primes.push(2 * i + 3);
        }
    }

    primes
}

fn toggle_fullscreen_and_clear_frame(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    enable_fullscreen: bool,
    background_color: Color,
) {
    {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(background_color);
    }

    if enable_fullscreen {
        let monitor_index = get_current_monitor();
        let monitor_w = get_monitor_width(monitor_index);
        let monitor_h = get_monitor_height(monitor_index);
        rl.set_window_size(monitor_w, monitor_h);
    } else {
        rl.set_window_size(WINDOW_WIDTH, WINDOW_HEIGHT);
    }

    rl.toggle_fullscreen();

    {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(background_color);
    }
}

#[derive(Copy, Clone, Debug)]
enum Theme {
    Default,
    Knight,
    NieR,
    Echidna,
    Solarized,
}

impl Theme {
    fn next(self) -> Theme {
        match self {
            Theme::Default => Theme::Knight,
            Theme::Knight => Theme::NieR,
            Theme::NieR => Theme::Echidna,
            Theme::Echidna => Theme::Solarized,
            Theme::Solarized => Theme::Default,
        }
    }

    fn colors(self) -> (Color, Color, Color, Color) {
                    // fg, bg, opts, hints 愛しています :3q
        match self {
            Theme::Default => (
                Color::new(102, 191, 255, 255),
                Color::new(0, 0, 0, 255),
                Color::new(255, 255, 255, 255),
                Color::new(115, 99, 86, 255),
            ),
            Theme::Knight => (
                Color::new(250, 251, 246, 255),
                Color::new(15, 15, 27, 255),
                Color::new(198, 183, 190, 255),
                Color::new(86, 90, 117, 255),
            ),
            Theme::NieR => (
                Color::new(157, 0, 6, 255),
                Color::new(194, 189, 166, 255),
                Color::new(25, 22, 6, 255),
                Color::new(7, 102, 120, 255),
            ),
            Theme::Echidna => (
                Color::new(204, 255, 153, 255),
                Color::new(17, 16, 16, 255),
                Color::new(242, 242, 242, 255),
                Color::new(126, 143, 126, 255),
            ),
            Theme::Solarized => (
                Color::new(65, 199, 185, 255),
                Color::new(0, 20, 25, 255),
                Color::new(173, 188, 188, 255),
                Color::new(24, 73, 86, 255),
            ),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Theme::Default => "Default",
            Theme::Knight => "Knight",
            Theme::NieR => "NieR",
            Theme::Echidna => "Echidna",
            Theme::Solarized => "Solarized",
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .resizable()
        .title("Espiral Arquimediana")
        .build();

    rl.set_target_fps(200);

    let mut fullscreen_enabled = false;

    let primes = sieve(SIEVE_LIMIT);
    let mut visible_primes_count = INITIAL_VISIBLE_PRIMES;

    let mut zoom_level = INITIAL_ZOOM_LEVEL;
    let mut rotation_offset = INITIAL_ROTATION_OFFSET;

    let mut paused   	 = false;
    let mut auto_zoom	 = true;
    let mut rotating	 = true;
    let mut show_hud	 = false;
    let mut show_tips	 = true;
    let mut reversed	 = false;
    let mut optimized	 = true;

    let mut theme = Theme::Default;

    while !rl.window_should_close() {
        let screen_width  = rl.get_screen_width() as f32;
        let screen_height = rl.get_screen_height() as f32;

        let (square_color, background_color, foreground_color, alt_foreground_color) = theme.colors();

        if rl.is_key_pressed(KeyboardKey::KEY_F11) {
            toggle_fullscreen_and_clear_frame(
                &mut rl,
                &thread,
                !fullscreen_enabled,
                background_color,
            );
            fullscreen_enabled = !fullscreen_enabled;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_T) {
            theme = theme.next();
	    optimized = match theme.name() {
		"NieR" => false,
		_ => true,
	    }
        }

        let wheel_step_normal = 0.12;
        let wheel_step_fine   = 0.03;
        let mouse_wheel_delta = rl.get_mouse_wheel_move();
        if mouse_wheel_delta != 0.0 {
            let ctrl_held = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
                || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
            let step = if ctrl_held { wheel_step_fine } else { wheel_step_normal };
            let zoom_multiplier = (1.0 + (mouse_wheel_delta as f64) * step).max(1e-12);
            zoom_level *= zoom_multiplier;
            zoom_level = zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
        }

	let arrow_step_normal = 0.03;
	let arrow_step_fine   = 0.01;
	let arrow_delta       = match (
	    rl.is_key_down(KeyboardKey::KEY_UP),
	    rl.is_key_down(KeyboardKey::KEY_DOWN),
	) {
	    (true, false) =>  1.0,
	    (false, true) => -1.0,
	    _ => 0.0,
	};
	if arrow_delta != 0.0 {
            let ctrl_held = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
                || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
            let step = if ctrl_held { arrow_step_fine / 5.0 } else { arrow_step_normal / 5.0 };
            let zoom_multiplier = (1.0 + (arrow_delta as f64) * step).max(1e-12);
	    zoom_level *= zoom_multiplier;
            zoom_level  = zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
	}

        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            break;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_A) {
            visible_primes_count = 150;
            zoom_level = INITIAL_ZOOM_LEVEL;
            rotation_offset = 0.0;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            visible_primes_count = primes.len();
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_Z) {
            auto_zoom = !auto_zoom;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_X) {
            rotating = !rotating;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_H) {
            show_hud = !show_hud;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_B) {
            show_tips = !show_tips;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            reversed = !reversed;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            optimized = !optimized;
        }

        if rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) {
            rl.show_cursor();
        } else {
            rl.hide_cursor();
        }

        if !paused {
            let primes_len = primes.len();
            let auto_zoom_factor = 0.9996;

            if !reversed {
                if visible_primes_count + PRIMES_INCREMENT <= primes_len {
                    visible_primes_count += PRIMES_INCREMENT;
                } else {
                    visible_primes_count = primes_len;
                }
                if auto_zoom && mouse_wheel_delta == 0.0 && arrow_delta == 0.0 {
                    zoom_level *= auto_zoom_factor;
                    zoom_level  = zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
                }
		if rotating {
		    rotation_offset += ROTATION_SPEED;
		}
            } else {
                visible_primes_count = visible_primes_count.saturating_sub(PRIMES_INCREMENT);
                visible_primes_count = std::cmp::max(visible_primes_count, 150);

                if auto_zoom {
                    zoom_level /= auto_zoom_factor;
                    zoom_level  = zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
                }
		if rotating {
		    rotation_offset -= ROTATION_SPEED;
		}
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(background_color);

        let center_x = 0.5f32 * screen_width;
        let center_y = 0.5f32 * screen_height;

	let max_square_initial_size = BASE_SQUARE_SIZE * (zoom_level as f32);
	let offscreen_margin_x = screen_width + max_square_initial_size * 4.0;
	let offscreen_margin_y = screen_height + max_square_initial_size * 4.0;

	for &prime in primes.iter().take(visible_primes_count) {
	    let angle    = prime as f64 + rotation_offset;
	    let radius   = prime as f64;
	    let (sin_t, cos_t) = angle.sin_cos();
	    let world_x  = radius * cos_t;
	    let world_y  = -radius * sin_t;

	    let screen_x = center_x + (world_x as f32) * (zoom_level as f32);
	    let screen_y = center_y + (world_y as f32) * (zoom_level as f32);

	    if screen_x < -offscreen_margin_x
		|| screen_x > screen_width + offscreen_margin_x
		|| screen_y < -offscreen_margin_y
		|| screen_y > screen_height + offscreen_margin_y
	    {
		continue;
	    }

	    let raw_size = BASE_SQUARE_SIZE * (zoom_level as f32);
	    let final_size = if optimized {
		raw_size.clamp(MIN_SQUARE_DISPLAY_SIZE_OPTIMIZED, MAX_SQUARE_DISPLAY_SIZE)
	    } else {
		raw_size.clamp(MIN_SQUARE_DISPLAY_SIZE, MAX_SQUARE_DISPLAY_SIZE)
	    };
	    let square_center = Vector2::new(screen_x, screen_y);
	    let square_size_vec = Vector2::new(final_size, final_size);
            if optimized && final_size <= MIN_SQUARE_DISPLAY_SIZE_OPTIMIZED {
                d.draw_pixel_v(square_center, square_color);
            } else {
                d.draw_rectangle_v(square_center, square_size_vec, square_color);
            }
	}

        if show_hud {
            d.draw_text(
                &format!("{}x{}", screen_width as i32, screen_height as i32),
                10,
                10,
                20,
                foreground_color,
            );
            d.draw_text(
                &format!("primos: {} de {}", visible_primes_count, primes.len()),
                10,
                40,
                20,
                foreground_color,
            );
            d.draw_text(
                &format!("zoom: {:.8}, offset: {:.3}", zoom_level, rotation_offset),
                10,
                70,
                20,
                foreground_color,
            );

            d.draw_text(&format!("invertido: {}", bool_pt(reversed)), 10, 100, 20, foreground_color);
            d.draw_text(&format!("otimizado: {}", bool_pt(optimized)), 10, 130, 20, foreground_color);
            d.draw_text(&format!("pausado: {}", bool_pt(paused)), 10, 160, 20, foreground_color);
            d.draw_text(&format!("rotacionando: {}", bool_pt(rotating)), 10, 190, 20, foreground_color);
            d.draw_text(&format!("zoom automático: {}", bool_pt(auto_zoom)), 10, 220, 20, foreground_color);
            d.draw_text(&format!("tela cheia: {}", bool_pt(fullscreen_enabled)), 10, 250, 20, foreground_color);
            d.draw_text(&format!("exibir dicas: {}", bool_pt(show_tips)), 10, 280, 20, foreground_color);
            d.draw_text(&format!("tema: {}", theme_name_pt(theme.name())), 10, 310, 20, foreground_color);

            if show_tips {
                d.draw_text(
                    "Atalhos:\n  A: simular desde o início\n  S: mostrar tudo\n  ESPAÇO: pausar/retomar\n  Z: alternar zoom automático\n  X: alternar rotação\n  H: alternar HUD\n  B: alternar dicas\n  R: alternar inversão\n  P: otimizar com pixels\n  T: trocar tema\n  F11: alternar tela cheia\n\n  CIMA/BAIXO/RODA DO MOUSE: aproximar/afastar\n  (segure CTRL para zoom preciso)\n\n  Segure ALT ESQUERDO para mostrar o cursor",
                    10,
                    340,
                    20,
                    alt_foreground_color,
                );
            }
        }
    }
}
