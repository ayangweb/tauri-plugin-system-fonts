import { useMount } from "ahooks";
import { Table } from "antd";
import { ColumnsType } from "antd/es/table";
import { toString } from "es-toolkit/compat";
import { useState } from "react";
import { SystemFont, getSystemFonts } from "tauri-plugin-system-fonts-api";

const App = () => {
  const [fonts, setFonts] = useState<SystemFont[]>([]);

  useMount(async () => {
    const fonts = await getSystemFonts();

    setFonts(fonts);
  });

  const render = (value: any, record: SystemFont) => {
    return (
      <div
        style={{
          fontFamily: record.fontName,
        }}
      >
        {toString(value)}
      </div>
    );
  };

  const columns: ColumnsType<SystemFont> = [
    {
      title: "Id",
      dataIndex: "id",
      render,
    },
    {
      title: "Name",
      dataIndex: "name",
      render,
    },
    {
      title: "Font Name",
      dataIndex: "fontName",
      render,
    },
    {
      title: "Weight",
      dataIndex: "weight",
      render,
    },
    {
      title: "Style",
      dataIndex: "style",
      render,
    },
    {
      title: "Monospaced",
      dataIndex: "monospaced",
      render,
    },
  ];

  return (
    <Table
      bordered
      loading={fonts.length === 0}
      rowKey="id"
      columns={columns}
      dataSource={fonts}
    />
  );
};

export default App;
