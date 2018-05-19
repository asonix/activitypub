# ActivityPub
__This crate defines the base set of types from the ActivityPub specification.__

## Usage
Add the following to your Cargo.toml
```toml
# Cargo.toml

activitypub = "0.1"
```

And then use it in your project
```rust
extern crate activitypub;
extern crate failure;
extern crate serde_json;

use activitypub::{context, object::Video};
use failure::Error;

fn run() -> Result<(), Error> {
    let mut video = Video::default();
    video.object_props.set_context_object(context())?;
    video.ap_object_props.set_likes_string("https://my-instance.com/likes".to_owned());

    let video_string = serde_json::to_string(&video)?;

    let video: Video = serde_json::from_str(&video_string)?;

    Ok(())
}
```

## Contributing
Feel free to open issues for anything you find an issue with. Please note that any contributed code will be licensed under the GPLv3.

## License

Copyright © 2018 Riley Trautman

ActivityPub is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

ActivityPub is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details. This file is part of ActivityPub.

You should have received a copy of the GNU General Public License along with ActivityPub. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
