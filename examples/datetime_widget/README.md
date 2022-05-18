# Custom widget

This example demonstrate a custom component where it can trigger a custom event,
which is plugged from the Application.

In this example we use a simplistic DateTime widget which triggers an event when
any changes to time or date should notify the Application.

Without custom component, we would have to manage the state of each of the underlying
2 input elements in our app, which otherwise should be delegated to a widget/component.
