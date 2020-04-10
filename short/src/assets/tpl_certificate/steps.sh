#!/bin/sh
region=us-east-1
deploy_stack_name=test1-bucket-template
template_name_file=certificate
stack_name=test1-certificate
domain_name=$1

aws --region $region cloudformation package \
                     --template-file ./$template_name_file.yaml \
                     --s3-bucket $deploy_stack_name \
                     --output-template-file ./$template_name_file.pkg.yaml

aws --region $region cloudformation deploy \
            --template-file ./certificate.pkg.yaml \
            --stack-name $stack_name \
            --parameter-overrides \
              DomainName=$domain_name
