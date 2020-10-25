<template>
  <div id="app" style="background-color:white;box-shadow:3px 3px 20px 0 purple;">
    <img alt="grpc logo" style="display:inline-block;width:200px;height:100px;vertical-align:middle;" src="./assets/grpc-horizontal-color.svg">
    <h3 style="display:inline-block;vertical-align:middle;margin-left:30px;font-family:Courier New;color:#244c5a;"> <span style="color:#5ac5c5;"> _></span> hello from grpc web!</h3>
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
  name: 'app'
}

</script>
