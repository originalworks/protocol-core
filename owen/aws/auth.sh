require() {
    command -v "$1" > /dev/null 2>&1 || {
        echo "Some of the required software is not installed:"
        echo "    please install $1" >&2;
        exit 4;
    }
}

require jq
require aws

unset AWS_SESSION_TOKEN
unset AWS_SECRET_ACCESS_KEY
unset AWS_ACCESS_KEY_ID

SERIAL_ARN=`aws iam list-mfa-devices | jq -r ".MFADevices[0].SerialNumber"`
echo $SERIAL_ARN
ROLE_ARN=`aws iam list-roles | jq -r '.Roles[] | select(.RoleName == "FullAccessAdminRole") | .Arn'`
echo $ROLE_ARN
echo -n "Enter code: "; read CODE

OUT=`aws sts assume-role --role-arn $ROLE_ARN --serial-number $SERIAL_ARN --role-session-name aaa --token-code $CODE --duration-seconds 3600`
export AWS_ACCESS_KEY_ID=`echo $OUT | jq -r ".Credentials.AccessKeyId"`
export AWS_SECRET_ACCESS_KEY=`echo $OUT | jq -r ".Credentials.SecretAccessKey"`
export AWS_SESSION_TOKEN=`echo $OUT | jq -r ".Credentials.SessionToken"`