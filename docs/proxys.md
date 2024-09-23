---
title: Docs with VitePress
editLink: true
---

<script setup>
import GetAddress from "./GetAddress.vue"

</script>

# Proxys

## 地质云

可通过 WMTS 链接加载

<GetAddress path="/WMTS/geocloud"/>

目前代理的地图有

- 基础地质图: 全国1/250万地质图
- 基础地质图: 全国1/150万地质图
- 基础地质图: 全国1/100万地质图
- 基础地质图: 全国1/50万地质图

::: danger ⚠️ 注意
地质云需要 token,请自行从官网获取  
公共 token 有时效性, 每几天需要更换新 token  
请在`config.toml`中填写 token
:::

## 吉林一号

::: danger ⚠️ 注意
‼️ 请先在`config.toml`中填写自己的 token
:::

可通过 WMTS 链接加载

<GetAddress path="/WMTS/jl1"/>

或 XYZ 链接加载

```text
http://127.0.0.1/getTile/{z}/{x}/{y}?mk=<地图mk>
```

> 2023年全国0.5米吉林一号影像 mk: `73ad26c4aa6957eef051ecc5a15308b4`
