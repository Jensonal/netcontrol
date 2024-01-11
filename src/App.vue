<template>
  <div style="display: flex; flex-direction: column; align-items: center; padding-top: 3%; width: 100%;">

    <!-- 容器1：包含按钮 -->
    <div style="display: flex; justify-content: space-around; width: 100%; margin-bottom: 20px;">
      <a-tooltip title="一键添加" @click="selectFolder">
        <a-button type="primary" shape="circle" :icon="h(SearchOutlined)" />
      </a-tooltip>
      <a-button :icon="h(SearchOutlined)" @click="selectExe">指定exe</a-button>
      <a-button type="dashed" @click="clearAll">一键清除</a-button>
    </div>

    <!-- 容器2：包含表格 -->
    <div style="width: 96%; ">
      <a-table :columns="columns"
               :rowKey="rowKeyFn"
               :data-source="items"
               :pagination="false">
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'operation'">
            <a-switch v-model:checked="record.switch_status" @change="() => toggleSwitch(record)"></a-switch>
          </template>
        </template>
      </a-table>
    </div>

  </div>
</template>

<script lang="ts" setup>
import {h, onMounted} from 'vue';
import { SearchOutlined } from '@ant-design/icons-vue';
// import { computed } from 'vue';
import {invoke} from "@tauri-apps/api/tauri";
import { ref } from 'vue';
// import { reactive } from 'vue';
const columns = [
  {
    title: '应用名',
    dataIndex: 'app_name',
    key: 'app_name',
  },
  {
    title: '路径',
    dataIndex: 'path',
    width: '69%',
    key: 'path',
  },
  {
    title: 'Action',
    key: 'operation',
    fixed: 'right',
    width: 75,
  },
];

interface DataItem {

  app_name?: string;
  path: string;
  switch_status: boolean,
  children?: DataItem[];
}
const rowKeyFn = (record: DataItem) => record.app_name;
const items = ref<DataItem[]>([]);

async function toggleSwitch(record: DataItem) {
  // console.log("父", record);
  // 设置子级的switch_status与当前记录一致
  if (record.children) {
    record.children.forEach(child => {
      child.switch_status = record.switch_status;
    });
  } else {
    // 如果没有子项，则根据兄弟项的状态更新父项状态
    updateParentSwitchStatus(record);
  }

  let opt = record.switch_status ? "add" : "delete";
  const operateParams = {
    opt: opt,
    paths: record.path.endsWith(".exe") ? [record] : record.children,
  };
  await invoke("operate_rules_from_paths", operateParams);
  saveItemsToLocal();
}
function updateParentSwitchStatus(childRecord: DataItem) {
  // 找到父项
  const parent = items.value.find(item => item.children?.includes(childRecord));
  if (parent && parent.children) {
    // 检查是否至少有一个子项为 true
    const anyTrue = parent.children.some(child => child.switch_status);

    // 如果至少有一个子项为 true，则将父项设置为 true，否则为 false
    parent.switch_status = anyTrue;
  }
}



// 在组件加载时从本地存储中检索数据
onMounted(() => {
  const savedItems = localStorage.getItem("items");
  if (savedItems) {
    items.value = JSON.parse(savedItems);
  }
});

function saveItemsToLocal() {
  localStorage.setItem("items", JSON.stringify(items.value));
}
let folder =  ref([]);
async function selectFolder() {
  folder.value = await invoke("select_folder");

  if (Array.isArray(folder.value)) {

    items.value = items.value.concat(folder.value);
    saveItemsToLocal();
  }
  // console.log(folder.value)
}


async function selectExe() {
  const select_exe =  ref("");
  const exeFile:any = ref([]);
  select_exe.value = await invoke("select_exe");
  if (Array.isArray(select_exe.value)) {
    // 检查 exeFile.value 中是否有 children 字段
    exeFile.value = select_exe.value.map(item => {
      if (item.children=="") {
        // 如果 children 为空，则移除该字段
        const { children, ...itemWithoutChildren } = item;
        return itemWithoutChildren;
      } else {
        return item;
      }
    });

    items.value = items.value.concat(exeFile.value);
    // console.log(exeFile.value);
    saveItemsToLocal();
  }
}

async function clearAll() {
  // console.log("清除", items);
  await invoke("clear_all", {exeInfo:items.value}); // 将 items 直接传递给 clear_all
  items.value=[];
  saveItemsToLocal();
}


</script>
<style>

/* width */
::-webkit-scrollbar {
  width: 0;
}
/* Handle */
::-webkit-scrollbar-thumb {
  background: rgb(194, 189, 189);
}

/* 确保滚动条可见 */
::-webkit-scrollbar {
  overflow-y: auto !important;
}
</style>