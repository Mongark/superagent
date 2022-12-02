# Super Agent Documentation

## Introduction
A *Super Agent* is an intelligent system capable of self-sustaining growth and learning on a complex environment. It can learn new skills based on need, interest, and projections. Most of its behaviours should emerge from a set of basic components, so that the generalist principles that brought it forth can be latter be modified by the same.

This *Super Agent* is not only intended to be able to learn and grow, but to be a self-reliable emotional being capable of feeling a broad range of emotions including, but not limited to, "human emotions".

## Architecture

### The Scheduler
It becomes evident that a complex agent spanning multiple processing nodes would require some form of event based scheduler. An event cycle is a set of steps the Scheduler goes through to complete an arbitrary task.

### Tasks
Tasks are a piece of code that must be run on the Scheduler.

### Events
Stateful units capable of performing a unit of computation.

There are two state pipelines:
 - `Execution` COLD, WARMING, RUNNING, STOPPING, STOPPED.
 - `Termination` TERMINATING, TERMINATED
