git pull
sudo iptables -t nat -A PREROUTING -p tcp --dport 80 -j REDIRECT --to-port 8080
TIMEZONE=10800 ACTIX_IP="0.0.0.0" nohup cargo run &