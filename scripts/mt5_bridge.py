#!/usr/bin/env python3
"""
MetaTrader5 Python Bridge

This script provides a Python interface to interact with MetaTrader5
and communicate with the Rust bot via REST API.
"""

import socket
import json
import logging
from datetime import datetime
from typing import Dict, List, Optional
import os
from dotenv import load_dotenv

load_dotenv()

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class MT5Bridge:
    """Bridge between Python and MetaTrader5"""
    
    def __init__(self, host: str = None, port: int = None):
        self.host = host or os.getenv('MT5_HOST', 'localhost')
        self.port = port or int(os.getenv('MT5_PORT', 5005))
        self.socket = None
        self.connected = False
    
    def connect(self, username: str = None, password: str = None) -> bool:
        """Connect to MetaTrader5"""
        try:
            username = username or os.getenv('MT5_USERNAME')
            password = password or os.getenv('MT5_PASSWORD')
            
            self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.socket.connect((self.host, self.port))
            
            # Send authentication
            auth_msg = {
                'action': 'auth',
                'username': username,
                'password': password
            }
            self.socket.sendall(json.dumps(auth_msg).encode())
            
            # Receive response
            response = self.socket.recv(1024).decode()
            response_data = json.loads(response)
            
            if response_data.get('status') == 'success':
                self.connected = True
                logger.info(f"Connected to MT5 at {self.host}:{self.port}")
                return True
            else:
                logger.error(f"Authentication failed: {response_data.get('error')}")
                return False
                
        except Exception as e:
            logger.error(f"Connection error: {e}")
            return False
    
    def get_account_info(self) -> Dict:
        """Get account information"""
        if not self.connected:
            logger.error("Not connected")
            return {}
        
        try:
            msg = {'action': 'get_account_info'}
            self.socket.sendall(json.dumps(msg).encode())
            response = self.socket.recv(4096).decode()
            return json.loads(response)
        except Exception as e:
            logger.error(f"Error getting account info: {e}")
            return {}
    
    def get_symbol_info(self, symbol: str) -> Dict:
        """Get symbol information"""
        if not self.connected:
            return {}
        
        try:
            msg = {'action': 'get_symbol_info', 'symbol': symbol}
            self.socket.sendall(json.dumps(msg).encode())
            response = self.socket.recv(4096).decode()
            return json.loads(response)
        except Exception as e:
            logger.error(f"Error getting symbol info: {e}")
            return {}
    
    def place_order(self, symbol: str, action: str, volume: float, 
                   price: float = 0, sl: float = 0, tp: float = 0) -> Dict:
        """Place an order"""
        if not self.connected:
            logger.error("Not connected")
            return {'status': 'error', 'message': 'Not connected'}
        
        try:
            msg = {
                'action': 'place_order',
                'symbol': symbol,
                'order_type': action,  # 'buy' or 'sell'
                'volume': volume,
                'price': price,
                'stop_loss': sl,
                'take_profit': tp,
                'comment': f'Bot8trade order at {datetime.now()}'
            }
            self.socket.sendall(json.dumps(msg).encode())
            response = self.socket.recv(4096).decode()
            return json.loads(response)
        except Exception as e:
            logger.error(f"Error placing order: {e}")
            return {'status': 'error', 'message': str(e)}
    
    def close_order(self, ticket: int, volume: float = 0) -> Dict:
        """Close an order"""
        if not self.connected:
            return {'status': 'error'}
        
        try:
            msg = {
                'action': 'close_order',
                'ticket': ticket,
                'volume': volume
            }
            self.socket.sendall(json.dumps(msg).encode())
            response = self.socket.recv(4096).decode()
            return json.loads(response)
        except Exception as e:
            logger.error(f"Error closing order: {e}")
            return {'status': 'error'}
    
    def get_positions(self) -> List[Dict]:
        """Get open positions"""
        if not self.connected:
            return []
        
        try:
            msg = {'action': 'get_positions'}
            self.socket.sendall(json.dumps(msg).encode())
            response = self.socket.recv(8192).decode()
            return json.loads(response).get('positions', [])
        except Exception as e:
            logger.error(f"Error getting positions: {e}")
            return []
    
    def disconnect(self):
        """Disconnect from MetaTrader5"""
        if self.socket:
            self.socket.close()
            self.connected = False
            logger.info("Disconnected from MT5")


if __name__ == '__main__':
    # Example usage
    bridge = MT5Bridge()
    
    if bridge.connect():
        # Get account info
        account = bridge.get_account_info()
        print(f"Account: {account}")
        
        # Get symbol info
        symbol_info = bridge.get_symbol_info('GOLD')
        print(f"GOLD Info: {symbol_info}")
        
        # Get positions
        positions = bridge.get_positions()
        print(f"Open Positions: {positions}")
        
        bridge.disconnect()
    else:
        print("Failed to connect to MT5")
