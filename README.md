A family of iteration traits which use move semantics.

# Move Semantics

Move semantics consume an iterator, and if there is a next item, a new iterator is returned
with the item, whereas when iteration completes, no new iterator is available. This prevents
incorrect states during type checking avoiding the need for runtime safety mechanisms like
[std::iter::FusedIterator].

# The Trait Family

There is a trait family based on two semantic axes: sync/async and the kind of
termination. Termination can be "endless", "finite", or "terminal", where "terminal" produces
a typed value as the final step. This leads to six traits:

|          | Sync                   | Async                       |
|----------|------------------------|-----------------------------|
| Endless  | [EndlessMoveIterator]  | [AsyncEndlessMoveIterator]  |
| Finite   | [FiniteMoveIterator]   | [AsyncFiniteMoveIterator]   |
| Terminal | [TerminalMoveIterator] | [AsyncTerminalMoveIterator] |

# Semantics At a Glance

Each trait is based on a single provided `into_next` method:

| Trait                       | Method |
|-----------------------------|---------------------------------------------------------------------------|
| [EndlessMoveIterator]       | `fn into_next(self) -> (Self, Self::Item);`                               |
| [FiniteMoveIterator]        | `fn into_next(self) -> Option<(Self, Self::Item)>;`                       |
| [TerminalMoveIterator]      | `fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal>;`       |
| [AsyncEndlessMoveIterator]  | `async fn into_next(self) -> (Self, Self::Item);`                         |
| [AsyncFiniteMoveIterator]   | `async fn into_next(self) -> Option<(Self, Self::Item)>;`                 |
| [AsyncTerminalMoveIterator] | `async fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal>;` |

# Producer/Consumer Precision

This family allows producing and consuming code to more precisely express semantics in
the type system. For example, a bigint sequence of all natural numbers could implement
`EndlessMoveIterator`, and a program which processes an asynchronous stream of child
process outputs and expects the child exit status as a terminal value would consume an
`AsyncTerminalMoveIterator`.

# Conversions

Producers and consumers may often have different iteration semantics, so this crate provides
conversions through blanket impls or *adapter* types.

## Converting into/from [std::iter::Iterator].

Each of the synchronous move iterator traits can potentially interact with
[std::iter::Iterator] consumer code:

### Blanket Impls

There are blanket impls of any [std::iter::Iterator] for any of the non-endless traits:

- [FiniteMoveIterator]
- [TerminalMoveIterator] with `Terminal = ()`
- [AsyncFiniteMoveIterator]
- [AsyncTerminalMoveIterator] with `Terminal = ()`

This allows any interfaces that accept these generic bounds to take a [std::iter::Iterator]
value seamlessly.

### Explicit Complete Conversions

These traits methods always convert into a [std::iter::Iterator]:

- [FiniteMoveIterator::into_iter]
- [EndlessMoveIterator::into_iter]

### Result Items/Terminators

Conversions between [TerminalMoveIterator] and [std::iter::Iterator] are only provided when
[Result] is involved to capture a common pattern:

Often [std::iter::Iterator] types have an [Result] with the untyped semantic
convention that the first `Err` item aborts iteration. By contrast to this convention, terminal
iterators can make this explicit in the type system:

- Result-item convention: `impl Iterator<Item = Result<T, E>>`
- Terminal type: `impl TerminalMoveIterator<Item = T, Terminal = Result<(), E>>`

Converting between these is provided by these functions:

- [terminal_move_iterator_from_result_iterator]
- [terminal_move_iterator_into_result_iterator]

# Adaptation

Several of the traits provide default methods to adapt them to provide a related trait via a
newtype wrapper. This helps consuming code and implementations to each tailor their semantics
precisely yet still interoperate, where the result has coherent semantics:

| Implemented Trait | Adaptation Method | Resulting Interface |
|-------------------|-------------------|---------------------|
| [EndlessMoveIterator]      | [into_async](EndlessMoveIterator::into_async)                                                    | [AsyncEndlessMoveIterator]  |
|                            | [into_finite_move_iterator](EndlessMoveIterator::into_finite_move_iterator)                      | [FiniteMoveIterator]        |
|                            | [into_terminal_move_iterator](EndlessMoveIterator::into_terminal_move_iterator)                  | [TerminalMoveIterator]      |
| [FiniteMoveIterator]       | [into_async](FiniteMoveIterator::into_async)                                                     | [AsyncFiniteMoveIterator]   |
|                            | [into_terminal_move_iterator](FiniteMoveIterator::into_terminal_move_iterator)                   | [TerminalMoveIterator]      |
| [TerminalMoveIterator]     | [into_async](TerminalMoveIterator::into_async)                                                   | [AsyncTerminalMoveIterator] |
| [AsyncEndlessMoveIterator] | [into_async_finite_move_iterator](AsyncEndlessMoveIterator::into_async_finite_move_iterator)     | [AsyncFiniteMoveIterator]   |
|                            | [into_async_terminal_move_iterator](AsyncEndlessMoveIterator::into_async_terminal_move_iterator) | [AsyncTerminalMoveIterator] |
| [AsyncFiniteMoveIterator]  | [into_async_terminal_move_iterator](AsyncFiniteMoveIterator::into_async_terminal_move_iterator)  | [AsyncTerminalMoveIterator] |

Adaptation with [TerminalMoveIterator::into_async] which propagates `Terminal` type, whereas
any other adaptation into a [TerminalMoveIterator] uses `()` as the `Terminal` type.

This provides only the minimal set of adaptations available between types, while the complete
transitive set of adaptions is provided by chaining calls. For example, to convert from an
[EndlessMoveIterator] into an [AsyncTerminalMoveIterator] chains two adaptations:

```
use moveiter::{EndlessMoveIterator, AsyncTerminalMoveIterator};

fn convert_emi_to_atmi<I>(emi: I) -> impl AsyncTerminalMoveIterator
where I: EndlessMoveIterator + Send + Sync,
{
use moveiter::AsyncEndlessMoveIterator; // pull in the latter method.

emi.into_async().into_async_terminal_move_iterator()
}
```

