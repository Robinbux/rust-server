import time
from locust import HttpUser, task

class QuickstartUser(HttpUser):
    @task
    def get_assets(self):
        self.client.get("/assets/pika")
        self.client.get("/assets/vid")
        self.client.get("/admin/console")

    '''@task(3)
    def view_item(self):
        for item_id in range(10):
            self.client.get("/admin/console", name="/admin")
            time.sleep(1)'''