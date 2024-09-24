git clone https://github.com/nityc-d-robo/robo2_main &&  
cd  robo2_main &&  
chmod +x ./start.sh &&  
cp ./start.sh ~/ &&  
(crontab -l 2>/dev/null; echo "@reboot /home/$USER/start.sh") | crontab -
