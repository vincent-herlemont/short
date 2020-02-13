#!/bin/sh
region=$1
stack_name=$2

if [ -z "$region" ]; then
  echo "region is not specified.";
  exit 1;
fi

if [ -z "$stack_name" ]; then
  echo "stack name is not specified."
  exit 1;
fi

aws --region $region cloudformation deploy \
                     --template-file ./bucket_template.yaml \
                     --stack-name "$stack_name" \
                     --parameter-overrides BucketName="$stack_name"

#aws --region $region cloudformation delete-stack --stack-name $stack_name