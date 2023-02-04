<template>
    <div class="box">
        <img class="shadow" src="../assets/img/babala.jpg" alt="囊哒哟!">
        <h1>来个大佬带带我!</h1>
    </div>
</template>

<script setup>

</script>

<style scoped>
.box {
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
.box img {
    height: 400px;
    width: 400px;
    border-radius: 20px;
}
.box h1 {
    font-size: 60px;
}
</style>