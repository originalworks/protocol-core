echo "Configuring CloudWatch..."
wget https://amazoncloudwatch-agent.s3.amazonaws.com/ubuntu/amd64/latest/amazon-cloudwatch-agent.deb
dpkg -i -E amazon-cloudwatch-agent.deb
log_group=$(jq -r '.Parameters.ValidatorLogGroupName' ../../template-config.json)
cat cloudwatch-config-template.json | sed "s/{{ValidatorLogGroupName}}/$log_group/g" > /opt/aws/amazon-cloudwatch-agent/bin/config.json
/opt/aws/amazon-cloudwatch-agent/bin/amazon-cloudwatch-agent-ctl \
    -a fetch-config \
    -m ec2 \
    -c file:/opt/aws/amazon-cloudwatch-agent/bin/config.json \
    -s