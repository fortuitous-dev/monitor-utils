=======================================================
Monitoring Utilities for Lightweight Collectors
=======================================================

These utilities are meant to bridge the gap between what Fluentbit already does,
and what it does not cover. We like using Fluentbit for log collection and monitoring,
and as such take the philosophy:

* Use Fluentbit for all monitoring and logging
* Logs can be used as metrics for alerting
* Only push data to collectors (like Loki) for secure firewall protection.


Disk Stats
===========
Fluentbit does alot already, but it seems to lack Disk statistics. It has a
*Disk Usage* module, but it only measures IO characteristics, not usage size
amounts. When disks fill up, systems tend to crash, so we really want to know
size data.

As such we provide a tool called disk_space to collect this.
It is meant to be called by Fluentbit's Exec input plugin::

    [INPUT]
        Name disk_space
        Tag  disk_stats
        Command /opt/bin/disk_space
        Interval_Sec 600
        Parser logfmt

    [OUTPUT]
        Name             loki
        Match            *
        URL              https://your-loki-server:3100/loki/api/v1/push
        Tenant_ID        <tenant-id>
        Basic_Auth_User  <username>
        Basic_Auth_Pass  <password>

Note that disk_space leverages the local *df* command and does not try to
re-implement a low-level too. It only works on Linux at this time.

Installation
================

* make install: This will all tools install it to the user ~/.cargo/bin/
  folder and to /opt/bin if you have permissions.

Testing
============

* make test
