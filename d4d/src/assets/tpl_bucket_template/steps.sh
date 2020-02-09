#!/bin/sh
region=us-east-1
stack_name=test2-bucket-template

aws --region $region cloudformation deploy \
                     --template-file ./bucket_template.yaml \
                     --stack-name "$stack_name" \
                     --parameter-overrides BucketName="$stack_name"
#aws --region $region cloudformation delete-stack --stack-name $stack_name