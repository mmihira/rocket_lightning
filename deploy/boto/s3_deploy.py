#!/usr/bin/python
import os, sys
import json
import boto
import boto.s3
import re
from boto.s3.key import Key
from boto.ec2.regioninfo import *

class Con:
    def __init__(self):
        self.proj_prefix = os.environ['PROJECT_PREFIX']

        f = open(os.path.expanduser(os.environ["{}_AWS_KEY_FILE".format(self.proj_prefix)]), 'r')
        credentials = json.loads(f.read())
        f.close()

        self.conn = boto.connect_s3(credentials['access_key'], credentials['secret_key'])
        bucket = self.conn.get_bucket('icnDb', validate=True)
        bucket_location = bucket.get_location()

        region = RegionInfo(name="ap-southeast-2", endpoint="ec2.ap-southeast-2.amazonaws.com")
        self.conn = boto.s3.connect_to_region(bucket_location,
                            aws_access_key_id=credentials["access_key"],
                            aws_secret_access_key=credentials["secret_key"])
        # print self.conn

    def percent_cb(self, complete, total):
        sys.stdout.write('.')
        sys.stdout.flush()

    def upload(self):
        bucket = self.conn.get_bucket('icnDb', validate=True)
        k = Key(bucket)

        base_path = os.environ['PARENT_DIRECTORY_OF_REPO']

        index_file = base_path + '/icnDb/app/index.html'

        print "Uploading index file"
        k.key = '/app/index.html'
        k.set_contents_from_filename(index_file, cb=self.percent_cb, num_cb=10)
        k.set_acl('public-read')
        print ''

        base_name = '/home/mihira/c/icnDb/app'
        dir = '/home/mihira/c/icnDb/app/pages'
        z = Key(bucket)
        for root, dirs, files in os.walk(dir):
            for name in files:
                abs_loc = os.path.join(root, name)
                print "\nUploading " + abs_loc
                base = '/app'+re.sub(base_name, '', root) + '/' + name
                z.key = base
                z.set_contents_from_filename(abs_loc, cb=self.percent_cb, num_cb=10)
                z.set_acl('public-read')

        base_name = '/home/mihira/c/icnDb/app'
        dir = '/home/mihira/c/icnDb/app/images'
        z = Key(bucket)
        for root, dirs, files in os.walk(dir):
            for name in files:
                abs_loc = os.path.join(root, name)
                print "\nUploading " + abs_loc
                base = '/app'+re.sub(base_name, '', root) + '/' + name
                z.key = base
                z.set_contents_from_filename(abs_loc, cb=self.percent_cb, num_cb=10)
                z.set_acl('public-read')

        print "\nUploading bundle file"
        bundle_file = base_path + '/icnDb/app/static/bundle.js'

        z = Key(bucket)
        z.key = '/app/static/bundle.js'
        z.set_contents_from_filename(bundle_file, cb=self.percent_cb, num_cb=10)
        z.set_acl('public-read')

        print "\n", k.generate_url(expires_in=0, query_auth=False)

    def test():
        for filename in os.listdir(directory):
            if filename.endswith(".asm") or filename.endswith(".py"):
                # print(os.path.join(directory, filename))
                continue
            else:
                continue


Con().upload()
