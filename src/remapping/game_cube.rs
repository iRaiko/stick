// remapping/game_cube.rs -- Stick
// Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

{
	fn remapper(input: (usize, ::Input)) -> (usize, ::Input) {
		(input.0, match input.1 {
			::Input::Move(x, y) => {
				::Input::Move(x / 0.6, y / 0.6)
			}
			::Input::Camera(x, y) => {
				::Input::Camera(x / 0.6, y / 0.6)
			}
			::Input::ThrottleL(x) => {
				::Input::ThrottleL(x / 0.7)
			}
			::Input::ThrottleR(x) => {
				::Input::ThrottleR(x / 0.7)
			}
			a => a
		})
	}

	::Remapper::new(0x791844, remapper)
}