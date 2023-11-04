#!/bin/bash

year=$(date +'%Y')
day=$(date +'%d')

cp -r _template "${year}/day-${day}"