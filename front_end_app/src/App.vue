<template>
  <div id="app" >
    <header style="background-color:white;box-shadow:3px 3px 20px 0 purple;">
      <img alt="grpc logo" style="display:inline-block;width:200px;height:100px;vertical-align:middle;" src="./assets/grpc-horizontal-color.svg">
      <h3 style="display:inline-block;vertical-align:middle;margin-left:30px;font-family:Courier New;color:#244c5a;"> <span style="color:#5ac5c5;"> _></span> hello from grpc web!</h3>
    </header>
    <div id="something" style="
      background-color:black;
      width: 800px;
      height: 300px;
      padding: 25px;
      margin: 25px;
      box-shadow:3px 3px 10px 0 rgba(0, 0, 0, 0.75);
      font-family:Courier New;
      color: white;"
      >
      <p> [ say something to grpc ] </p>
      <p> grpc@web # <input autofocus style="background-color:black;border:none;outline:none;font-family:Courier New;color: white;" v-model="message"> </p>
      <button style="background-color:black;border:1px solid #244c5a;padding: 10px 10px;outline:none;margin: 5px 5px;font-family:Courier New;color:white;" v-on:click="grpc()"> --> send request</button>
      <p>{{ grpcResponse }}</p>
    </div>
  </div>
</template>

<script>
import { HelloServiceClient } from '../../proto/output/HelloServiceClientPb'
import { SayHelloRequest } from '../../proto/output/hello_pb'

const hello = new HelloServiceClient('http://0.0.0.0:9091', null, null)
const request = new SayHelloRequest()
const name = 'gRPC!'
request.setName(name)

const metadata = { 'grpc-web-text': '*' }
hello.sayHello(request, metadata, (err, response) => {
  if (err) {
    console.log(err)
  } else {
    console.log(response.getMessage())
  }
})

export default {
  name: 'app',
  data: function () {
    return {
      message: null,
      grpcResponse: null
    }
  },
  methods: {
    grpc: function () {
      const hi = new HelloServiceClient('http://0.0.0.0:9091', null, null)
      const req = new SayHelloRequest()
      const n = this.message
      req.setName(n)
      const md = { 'grpc-web-text': '*' }
      hi.sayHello(req, md, (err, response) => {
        if (err) {
          this.grpcResponse = err
        } else {
          this.grpcResponse = response.getMessage()
        }
      })
    }
  }
}

</script>
