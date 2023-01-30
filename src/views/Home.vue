<template>
    <div class="home">
        <img class="shadow" src="../assets/img/klee.jpg" alt="囊哒哟!">
        <h1>欢迎使用Genshin Project Tools!</h1>
    </div>
</template>

<script setup>

</script>

<style scoped>
.home {
    min-height: calc(100vh - 35px);
    width: 100%;
    display: flex;
    box-sizing: border-box;
    padding: 20px 20px 20px 20px;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    gap: 3.125rem;
    line-height: 60px;
}
.home h1 {
    font-size: 60px;
}
.home img {
    height: 400px;
    width: 400px;
    border-radius: 20px;
}
@media (prefers-color-scheme: Dark) {
}
</style>