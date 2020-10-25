import { expect } from 'chai'
import { shallowMount } from '@vue/test-utils'
import App from '@/App.vue'

describe('App', () => {
  it('should work', () => {
    const wrapper = shallowMount(App)
    expect(wrapper.text()).to.include('_> hello from grpc web!')
  })
})
