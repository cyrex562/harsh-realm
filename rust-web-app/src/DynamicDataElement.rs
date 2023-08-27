// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.DynamicDataElement
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class DynamicDataElement
  {
    pub DynamicType type;
    pub fontName: String;
    pub fontSize: i32;
    pub fontStyle: i32;
    pub lineHeight: i32;
    pub color: Color;
    pub eventPicture: i32;
    pub historicalUnitPortrait: i32;
    pub texty: String;
    pub image: String;
    pub x: i32;
    pub y: i32;
    pub w: i32;
    pub h: i32;
    pub z: i32;
    pub optionpp: i32;
    pub optionevent: i32;
    pub optiontitle: String;
    pub optiontext: String;
    pub optiontempvar: Vec<i32>;
    pub optiontempvarOn: Vec<bool>;
    pub center: bool;

    pub DynamicDataElement()
    {
      this.optiontempvar = new int[500];
      this.optiontempvarOn = new bool[500];
      this.color = Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
    }
  }
}
