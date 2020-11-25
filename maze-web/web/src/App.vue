<template>
  <div id="nav">
    <div id="options-toggle">
      <button @click="optionsOpen=!optionsOpen">{{ showOptionsLabel }}</button>
    </div>
    <button>Save</button>

  </div>
  <div id="body">
    <Main :optionsOpen="optionsOpen"/>
  </div>
</template>

<script>
import Main from './components/Main.vue'

export default {
  name: 'App',
  components: {
    Main
  },
  computed: {
    showOptionsLabel () {
      return this.optionsOpen ? '<<' : '>>'
    }
  },
  data () {
    return {
      optionsOpen: false
    }
  }
}
</script>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}

#options-toggle {
  display: inline-block;
  margin-right: 1em;
  button {
  border: none;
  padding: 0.5em 1em;
  cursor: pointer;

   &:hover {
     text-decoration: underline;
   }
  }

}

</style>
