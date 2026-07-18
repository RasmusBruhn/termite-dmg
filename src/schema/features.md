# Schema support list

This is all the schema features which should be supported, using schema version
2020-12 (https://www.learnjsonschema.com/2020-12/core/schema/).

## Supported (export)

This is all the keywords in a schema to support which may be useful.

### $id

The URI id for a schema, should probably be the same as type names

### $schema

Always set to "https://json-schema.org/draft/2020-12/schema", should warn if
input file does not use this.

### $ref

Used for referencing other schema files.

### $comment

Just comments, should be used to write that it was generated with termite-dmg.

### $drefs

Used to store sub schemas which can be references, should be used to save all
type data.

### $anchor

Static string to allow references to schema, maybe not needed or else same as type name

### $dynamicAnchor, $dynamicRef

Just like anchor/ref but can be used to override default implementations from
other files, maybe useful in inheritance, maybe not.

### allOf

List of schemas which must all be valid, maybe usefull for inheritance.

### anyOf

List of schemas where at least one must be valid, useful for variants.

### properties

List of fields and their schemas, useful for structs.

### additionalProperties

A schema which all fields not in 'properties' or 'patternProperties' must match
against. Could be useful for enums to set to false.

### propertyNames

A schema which all fields of the struct must match against. Could be useful for
enums.

### items

A schema which each item of an array must match against, useful for arrays.

### type

Type of object or if an array then the type must be included, useful for all and
can be imported as variant.

### enum

Restricts an object to be one of a set of values, could be useful for
constrained type or in combination with 'propertyNames' for enums.

### maxLength, minLength

Maximum/minimum number of characters in string, useful for constrained types.

### pattern

A regex that a string must match against, useful for constrained types.

### exclusiveMaximum, exclusiveMinimum, maximum, minimum

Bounds for an integer or number. Useful for constrained type.

### multipleOf

A number or integer must be a multiple of this value.

### maxProperties, minProperties

The maximum/minimum number of fields in a struct, maybe useful for enum to set
'maxProperties' and 'minProperties' to 1.

### required

A list of fields for struct that must occur, useful for structs.

### uniqueItems

If set to true then all items in array must be unique.

### description

The description of the object.

### default

A default value for an object, useful for structs.

### deprecated

Whether the feature is deprecated, we should add that as well.

### examples

A list of examples for an object.

## Supported (no export)

This is all the keywords in a schema to support which will not be useful but
still supported for import.

### not

Validates true if sub schema is not valid. May be supported for import with some
constrained types.

### title

The title of the object, can probably be inserted into the description.

## Unsupported

This is all the keywords in a schema to support which will not be useful or
supported.

### oneOf

A list of schemas where exactly one must be valid, cannot be supported.

### if, then, else

Each has a schema attached, if the 'if' schema passes, then use the 'then'
schema, else use the 'else' schema.

### patternProperties

A set of schemas like 'properties', but here it is not a single field, but any
field that matches against it.

### dependentSchemas

A list of field names and schemas, if a field from this list is defined then the
schema must also be valid.

### propertyNames

A schema which all fields of the struct must match against.

### contains

A schema which at least one element of an array must match against.

### prefixItems

Same as items but describes schemas that each the first elements of an array
must match.

### const

Restricts an object to be equal to one specific value

### dependentRequired

A map of fields that if they occur require other fields to exist

### maxContains, minConstains

The maximum/minimum number of times an array element can match a schema.

### readOnly, writeOnly

Whether the object is read/write only.

## ConstrainedTypes

- MinLength (array, string)
- MaxLength (array, string)
- Enum (all)
- Pattern (string)
- Format (string) (like version, date, ...)
- ExclusiveMaximum (number, integer)
- ExclusiveMinimum (number, integer)
- Maximum (number, integer)
- Minimum (number, integer)
- MultipleOf (integer)
- Unique (array)

