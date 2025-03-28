---
title: Proxys
---

<script setup>
import GetAddress from "./GetAddress.vue"

</script>

# Proxys

## 吉林一号

::: warning ⚠️ 注意
‼️ 使用之前，请先在请在 `Setting` 页面或`config.toml`中填写自己的 token  
访问 [吉林 1 号官网](https://www.jl1mall.com/rskit/MyRSservice) 获取 TK
:::

Arcmap / Arcgis Pro 可通过 WMTS 服务加载

WMTS服务地址:
<GetAddress path="/WMTS/jl1"/>

可在`config.toml`中自定义mk，如：
```toml
[jl1_mk]
73ad26c4aa6957eef051ecc5a15308b4 = "2023年度全国高质量一张图"
92e687bbb824b4def35cfab91c9103d0 = "2024年度全国高质量一张图"
2d9bf902749f1630bc25fc720ba7c29f = "2022年度全球一张图"
```

XYZ 服务地址:
<GetAddress path="/getTile/jl1/{z}/{x}/{y}?mk=73ad26c4aa6957eef051ecc5a15308b4"/>

> mk值可自行更换。


### 视频教程:
<iframe src="//player.bilibili.com/player.html?isOutside=true&aid=113406514301152&bvid=BV144SdYiECo&cid=26561089267&p=1&autoplay=0" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true" width="100%" height="387px"></iframe>

## 地质云

::: warning ⚠️ 注意
地质云需要 token,请自行从[官网](https://igss.cgs.gov.cn/admin/token/index.jsp)申请获取  
请在 `Setting` 页面或者`config.toml`中填写 token
:::

可通过 WMTS 服务加载

WMTS服务地址:
<GetAddress path="/WMTS/geocloud"/>

目前代理的地图有

- 基础地质图: 全国 1：250 万地质图
- 基础地质图: 全国 1：150 万地质图
- 基础地质图: 全国 1：100 万地质图
- 基础地质图: 全国 1：50 万地质图
- 基础地质图: 全国 1：20 万地质图

> 视频演示:
<iframe src="//player.bilibili.com/player.html?isOutside=true&aid=113163043409358&bvid=BV17jtWetEvB&cid=25919818527&p=1&autoplay=0" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true" width="100%" height="387px"></iframe>

## 常用 XYZ 服务转 WMTS

> 直连无代理

支持以下地图:

- Open Street Map
- Google Map

可通过 WMTS 链接加载
<GetAddress path="/WMTS/XYZ"/>

::: info
🤔 适合在 Arcmap 中使用
:::
