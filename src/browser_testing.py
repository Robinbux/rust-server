from selenium.webdriver import Firefox

driver = Firefox(executable_path='/opt/WebDriver/bin/geckodriver')

try:
    driver.get("http://localhost:8087/home")

finally:
    driver.quit()