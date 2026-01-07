export interface ApiItem {
  id: string;
  name: string;
  description: string;
  path: string;
  category: string;
}

export interface ApiCategory {
  id: string;
  name: string;
  icon: string;
  color: string;
  items: ApiItem[];
}

export const apiCategories: ApiCategory[] = [
  {
    id: "data-processing",
    name: "数据处理",
    icon: "🔄",
    color: "primary",
    items: [
      {
        id: "replace-undefined",
        name: "替换 Undefined",
        description: "将 JavaScript 对象中的 undefined 值替换为 null",
        path: "/docs/data-processing/replace-undefined",
        category: "data-processing"
      }
    ]
  },
  {
    id: "text-conversion",
    name: "文本转换",
    icon: "🔤",
    color: "secondary",
    items: [
      {
        id: "digit-uppercase",
        name: "数字转大写",
        description: "将阿拉伯数字转换为中文大写格式",
        path: "/docs/text-conversion/digit-uppercase",
        category: "text-conversion"
      }
    ]
  }
];

export function getAllApis(): ApiItem[] {
  return apiCategories.flatMap(category => category.items);
}

export function getApiById(id: string): ApiItem | undefined {
  return getAllApis().find(api => api.id === id);
}

export function getCategoryById(id: string): ApiCategory | undefined {
  return apiCategories.find(category => category.id === id);
}
