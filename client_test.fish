#!/bin/fish

set output_file /tmp/hoarder_dev_test

set url 'http://localhost:8000'

curl -X GET -d '{"user_fingerprint": "5019 B899 74E8 F56C 2B5B  FA4F EBF8 5543 5E77 07A1"}' -H 'Content-Type: application/json' $url/login --silent > $output_file

set api_token ( gpg -r -a -d -q $output_file)

curl -X GET -d '{"user_fingerprint": "5019 B899 74E8 F56C 2B5B  FA4F EBF8 5543 5E77 07A1","api_token": "'$api_token'"}' -H 'Content-Type: application/json' $url/secret --silent > $output_file

gpg -r -a -d -q $output_file
