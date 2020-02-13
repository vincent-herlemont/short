#!/bin/sh
region=eu-west-3
deploy_bucket_name=sam-test-template
stack_name=$1
template_name=template
target=$2

if [ -z "$stack_name" ]; then
  echo "no stack name specified";
  exit 1;
fi

# shellcheck disable=SC2039
if [ "$target" == "cf" ]; then
  aws --region $region cloudformation package \
       --template-file ./$template_name.yaml \
       --template-file ./$template_name.yaml \
       --s3-bucket $deploy_bucket_name \
       --output-template-file ./$template_name.pkg.cf.yaml

  aws --region $region cloudformation deploy \
              --template-file ./$template_name.pkg.cf.yaml \
              --stack-name $stack_name \
              --capabilities CAPABILITY_IAM
elif [ "$target" == "sam" ]; then

    sam package \
      --template-file $template_name.yaml \
      --output-template-file $template_name.pkg.sam.yaml \
      --s3-bucket $deploy_bucket_name

    sam publish \
      --template $template_name.pkg.sam.yaml \
      --region $region

elif [ "$target" == "sam-local" ]; then
    sam local start-api --region $region
else
  echo "no target specified !";
  exit 1;
fi