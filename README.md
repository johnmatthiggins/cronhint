# Cron Cli Helper

`cronhint` is a Crontab.guru inspired CLI tool that translates CRON expressions to plain english.

## Examples

```
$ cronhint '4 1 * * *'
At 1:04 AM.
```

```
$ cronhint '4 * * 1-3 1'
At minute 4 every hour on Monday in every month from January through March
```

```
$ cronhint '* 0 */2 8 *'
At every minute past 12:00 AM on every 2nd day-of-month in August.
```

```
$ cronhint '0 12 25 12 *'
At 12:00 PM on day-of-month 25 in December.
```
