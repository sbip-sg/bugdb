# Prerequisite: `bs4`
#   - pip install bs4

import requests
import urllib.request
import signal
import time
from bs4 import BeautifulSoup

def extract_swc_info(swc_id):
    url = 'https://swcregistry.io/docs/SWC-' + str(swc_id)
    response = requests.get(url)
    soup = BeautifulSoup(response.text, "html.parser")

    # Title
    title_id = soup.findAll("a", {"id" : "title"})
    title_node = title_id[0].find_parent()
    title_content = title_node.find_next_sibling().text
    print("- Title: " + title_content)

    # Relationships
    relationship_id = soup.findAll("a", {"id" : "relationships"})
    relationship_node = relationship_id[0].find_parent()
    relationship_link = relationship_node.find_next_sibling()
    relationship_content = relationship_node.find_next_sibling().text
    print("- Relationship link: " + str(relationship_link.find_all("a")[0]))
    print("- Relationships: " + relationship_content)

    # Description
    description_id = soup.findAll("a", {"id" : "description"})
    description_node = description_id[0].find_parent()
    description_content = description_node.find_next_sibling().text
    print("- Description: " + description_content)

    # Remediation
    remediation_id = soup.findAll("a", {"id" : "remediation"})
    remediation_node = remediation_id[0].find_parent()
    remediation_content = remediation_node.find_next_sibling().text
    print("- Remediation: " + remediation_content)


# Main function
def main():
    swc_id = 100
    extract_swc_info(swc_id)

if __name__ == "__main__":
    main()
