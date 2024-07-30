# rusty-cron
Lightweight and performant library to parse cron format strings and return the remaining miliseconds to execute.

1. [About](#about)
2. [Features](#features)
3. [Installation](#installation)
4. [Usage](#usage)
5. [Contributing](#contributing)

## About
rusty-cron is a simple library made to parse cron formatted strings and return the remaining milliseconds to the next execution, either using current time or a given one,
it's made to be very quick at the cost of being a bit more heavy on memory, but it shouldn't be a problem for most applications.

## Features
Currently allows for either a 5 or 6 tokens cron formated string depending on if you need the precision to go up to minutes or seconds respectively.
It also allows all normal tokens to be used in the string, those are:

- *: Defines all possible values
- x/y: Defines values starting at x and repeating every y, for example: 0/5, every 5 minutes starting at 0
- x-y: Values starting at x until y, for example: 5-15, every minute between 5 and 15
- x,y,z: Specific values defined, for example: 1,5,50, at minutes 1, 5 and 50

You can also mix and match those in some ways, checkout https://crontab.guru/ for an amazing explanation on what does your cron do.

## Installation
Should be as easy as **cargo add rusty-cron**
Or do a local installation by defining the path where your code is like **rusty-cron= { path = "../rusty-cron" }**

## Usage
Simply add the use rusty_cron::Cron and use Cron::parse_time() with your cron string (I recommend https://crontab.guru/ to validate your strings and create new ones),
and, optionally, a second parameter DateTime<Utc> to give the algorithm the starting date, by default it'll be "chrono::Utc::now()".

That'll return a result, with a string with the error message or the milliseconds remaining to the next theoretical execution.
If what you want is for a task to be triggered periodically, have a look at rusty-scheduler, it uses this same cron string format to automatically execute functions.

## Contributing
If you feel like something is missing, could be improved or needs to be changed let me know on a ticket or just start a pull request and I'll try to have a look at it asap.
