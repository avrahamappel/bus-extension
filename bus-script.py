#!/usr/bin/env python3

import json
import math
import os
import requests

from bs4 import BeautifulSoup

# Function to calculate the distance between two latitude/longitude points
def haversine(lat1, lon1, lat2, lon2):
    R = 6371000  # Radius of the Earth in meters
    phi1 = math.radians(lat1)
    phi2 = math.radians(lat2)
    delta_phi = math.radians(lat2 - lat1)
    delta_lambda = math.radians(lon2 - lon1)

    a = math.sin(delta_phi / 2) ** 2 + math.cos(phi1) * math.cos(phi2) * math.sin(delta_lambda / 2) ** 2
    c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))

    return R * c  # Distance in meters

# Test for haversine function
def test_haversine():
    point1 = (34.0522, -118.2437)  # Los Angeles
    point2 = (34.0522 + 0.0045, -118.2437)  # Slightly east
    expected_distance = 500  # Approximate distance in meters

    calculated_distance = haversine(point1[0], point1[1], point2[0], point2[1])
    assert abs(calculated_distance - expected_distance) < 10, f"Expected ~{expected_distance}m but got {calculated_distance}m"

# Run the test
testing = False
if testing:
    test_haversine()
    print("Test passed.")
    exit(0)

login_url = "https://tstg.mybusplanner.ca/Login"
bus_tracker_url = "https://tstg.mybusplanner.ca/Subscriptions/WheresMyBus"

BUS_USERNAME = os.getenv('BUS_USERNAME')
BUS_PASSWORD = os.getenv('BUS_PASSWORD')
headers = {
    'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:145.0) Gecko/20100101 Firefox/145.0'
}
login_payload = {
    'ctl00$MainContent$glogin$lLogin$UserName': BUS_USERNAME,
    'ctl00$MainContent$glogin$lLogin$Password': BUS_PASSWORD,
}

# Log into the site
session = requests.Session()
session.get(login_url, headers=headers)
print(session.cookies)
login_response = session.post(login_url+"?LoginType=Subscriber", data=login_payload, headers=headers) 
login_response.raise_for_status()
if login_response.status_code != 302:
    print("Not logged in")
    exit(1)

while True:
    response = session.get(bus_tracker_url, headers=headers)
    response.raise_for_status()  # Raise an exception for HTTP errors

    # Parse the HTML response
    soup = BeautifulSoup(response.text, 'html.parser')

    # Extract latitude and longitude from hidden inputs
    bus_position_json = soup.find('input', {'id': 'MainContent_NestContent_hfBusLocation'})['value']
    stop_positions_json = soup.find('input', {'id': 'MainContent_NestContent_hfBusStopLocations'})['value']
    bus_position = json.loads(bus_position_json)
    stop_positions = json.loads(stop_positions_json)
    bus_lat = bus_position["Latitude"]
    bus_lon = bus_position["Longitude"]
    stop_lat = stop_positions[0]["Latitude"]
    stop_lon = stop_positions[0]["Longitude"]

    # Calculate the distance
    distance = haversine(stop_lat, stop_lon, bus_lat, bus_lon)

    # Check if the position is within 500 meters
    if distance <= 500:
        print(f"Position ({returned_lat},{returned_lon}) is within 500 meters.")
        subprocess.run(['toastify', 'BusPlanner', 'Bus is within 500m of your stop!'])
        break

    time.sleep(1)  # Wait for 1 second before retrying
    print(f"Position ({returned_lat},{returned_lon}) is outside 500 meters. Retrying...")
