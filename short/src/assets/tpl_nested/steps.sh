#!/bin/sh
region=`eu-west-3`
deploy_stack_name=test1-bucket-template
template_name_file=main
stack_name=$1

aws --region $region cloudformation package \
                     --template-file ./$template_name_file.yaml \
                     --s3-bucket $deploy_stack_name \
                     --output-template-file ./$template_name_file.pkg.yaml

aws --region $region cloudformation deploy \
            --template-file ./$template_name_file.pkg.yaml \
            --stack-name $stack_name \
            --capabilities CAPABILITY_IAM
