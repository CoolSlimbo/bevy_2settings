# Notes:
- Predicate and default must be of type `Result<T, BevyError>`.
- Serde `Serialize` features can be added directly to the field. `Deserialize` has to be added via the serde parameter.

# Plan:
- Add built in UI "components", that can easily interface with settings.
- Examples:
    - Checkbox
    - Dual button select (radio)
    - Text Box w/ filterable inputs - TODO:
    - Scrollable list
    - Selectable Options