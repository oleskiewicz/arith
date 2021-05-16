# arith
Containers with arithmetic operations support.

## Usage

	arithmap!{"a" => 1, "b" => 2} + 1 == arithmap!{"a" => 2, "b" => 3};
	arithmap!{"a" => 1, "b" => 2} + arithmap!{"b" => 2, "c" => 3} == arithmap!{"a" => 1, "b" => 4, "c" => 3};
	(arithmap!{"a" => 0, "b" => 1}.prune()) == arithmap!{"b" => 1};

You can access underlying values with `.hashmap` field.  For now only `&'a str` keys are
supported, until I convince the borrow checker to let me use a generic key type.


## Documentation

* [API docs](https://doc.oleskiewi.cz/arith/arith)


## Copyright

Copyright (C) 2021- Piotr Oleśkiewicz

This program is free software: you can redistribute it and/or modify it under the terms
of the GNU General Public License as published by the Free Software Foundation, either
version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.  See the GNU General Public License for more details.
