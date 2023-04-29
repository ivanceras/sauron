## Architecture
Note: This doc adapted from elm-guide

Sauron follows the Model-view-update architecture (also known as The Elm Architecture).
Sauron program produces HTML to show on screen and then the computer sends back
messages of what is going on. "They clicked a button!"

What happens within the Sauron program though? It always breaks into three parts:

- Model - the state of your application
- View - a way to turn your state into HTML
- Update - a way to update your state based on messages

These three concepts are the core of Model-view-update architecture.


## Application and Components

In order for your Model to be run in a sauron Program it has to implement the Application trait.
The Application trait requires you to define a `view` function which tells the program how to display the Model,
and an `update` function which describes how the Model state is updated based on the messages.

