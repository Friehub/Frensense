// Vulnerable: Possibility of prototype polluting assignment detected. By adding or modifying attributes of an object prototype, it is possible to create attributes that exist on every object, or replace critical attributes with malicious ones. This can be problematic if the software depends on existence or non-existence of certain attributes, or uses pre-defined attributes of object prototype (such as hasOwnProperty, toString or valueOf). Possible mitigations might be: freezing the object prototype, using an object without prototypes (via Object.create(null) ), blocking modifications of attributes that resolve to object prototype, using Map instead of object.
// Pattern: $X[$B] = ...
function vulnerable() {
  // TODO: implement pattern match
}
