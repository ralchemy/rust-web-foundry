# Task Reference Context

Rust Web Foundry uses a canonical Task slice as executable architecture documentation. The language below belongs to that reference slice; it is not a universal task-management model.

## Language

**Task candidate**:
A proposed Task whose supplied fields are valid but which the Task Policy has not yet accepted. It has no Task identity.
_Avoid_: Draft Task

**Task**:
An accepted unit of intended work with its own identity and canonical title, description, priority, assignee, and estimate values.
_Avoid_: Todo, job

**Task Policy**:
The acceptance authority that allows or rejects a Task candidate before Task creation. It does not rewrite the candidate or create the Task.
_Avoid_: Task validator

**Assignee**:
The identity optionally associated with a Task candidate or Task.
_Avoid_: Owner

**Task Priority**:
The Task's finite importance category: `low`, `normal`, or `high`.
_Avoid_: Severity

**Task Estimate**:
The optional expected effort for a Task, expressed in minutes.
_Avoid_: Deadline
