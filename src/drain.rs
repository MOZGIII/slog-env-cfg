use slog::{Drain, Level, Never, OwnedKVList, Record};

pub(crate) enum EitherDrain<Left, Right>
where
    Left: Drain<Ok = (), Err = Never>,
    Right: Drain<Ok = (), Err = Never>,
{
    Left(Left),
    Right(Right),
}

impl<Left, Right> Drain for EitherDrain<Left, Right>
where
    Left: Drain<Ok = (), Err = Never>,
    Right: Drain<Ok = (), Err = Never>,
{
    type Ok = ();
    type Err = Never;

    #[inline]
    fn log(
        &self,
        record: &Record<'_>,
        values: &OwnedKVList,
    ) -> std::result::Result<Self::Ok, Self::Err> {
        match self {
            EitherDrain::Left(drain) => drain.log(record, values),
            EitherDrain::Right(drain) => drain.log(record, values),
        }
    }

    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        match self {
            EitherDrain::Left(drain) => drain.is_enabled(level),
            EitherDrain::Right(drain) => drain.is_enabled(level),
        }
    }
}
