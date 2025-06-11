#!/bin/bash

application_home() {
	echo $(curl 0.0.0.0:3000)
}

application_home
