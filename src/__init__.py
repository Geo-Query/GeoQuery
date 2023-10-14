from flask import Flask,render_template,request

app = Flask(__name__)

@app.route("/")
def home():
    return render_template("form.html")

@app.route("/data/",methods=['POST','GET'])
def data():
    if request.method == 'GET':
        return f"Go to home page"
    if request.method == 'POST':
        form_data = request.form
        return render_template('data.html',form_data = form_data)

