#[macro_export]
macro_rules! any_of {
    ($state: expr, $($ident: ident),*) => {
        {
            $(
                let $ident = match $state.parse() {
                    Ok(ret) => return Ok(ret),
                    Err(err) => err,
                };
            )*

            let errors = vec![
                $($ident.kind),*
            ];

            Err($state.error(errors))
        }
    };

    ($state: expr, $($ident: ident => $map: block),*) => {
        {

            $(
                let $ident = match $state.parse::<$ident>().map(|_| $map) {
                    Ok(ret) => return Ok(ret),
                    Err(err) => err,
                };
            )*

            let errors = vec![
                $($ident.kind),*
            ];

            Err($state.error(errors))
        }
    };

    ($state: expr, $($ident: ident => $item: ident $map: block),*) => {
        {

            $(
                let $ident = match $state.parse::<$ident>().map(|$item| $map) {
                    Ok(ret) => return Ok(ret),
                    Err(err) => err,
                };
            )*

            let errors = vec![
                $($ident.kind),*
            ];

            Err($state.error(("", errors)))
        }
    };


}
