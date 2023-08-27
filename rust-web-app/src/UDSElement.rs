// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UDSElement
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class UDSElement
  {
    pub UDSType type;
    pub fontName: String;
    pub fontSize: i32;
    pub fontStyle: i32;
    pub lineHeight: i32;
    pub color: Color;
    pub eventPicture: i32;
    pub bitmapSlot: i32;
    pub historicalUnitPortrait: i32;
    pub texty: String;
    pub image: String;
    pub group: i32;
    pub x: i32;
    pub y: i32;
    pub w: i32;
    pub h: i32;
    pub z: i32;
    pub ox: i32;
    pub oy: i32;
    pub oh: i32;
    pub ow: i32;
    pub optionpp: i32;
    pub optionevent: i32;
    pub optiontitle: String;
    pub optiontext: String;
    pub optiontempvar: Vec<i32>;
    pub optiontempvarOn: Vec<bool>;
    pub eventNr: i32;
    pub grayed: i32;
    pub key: String;
    pub value: String;
    pub mouseOver: String;
    pub flagged: bool;
    pub smallgfx: i32;
    pub minvalue: i32;
    pub maxvalue: i32;
    pub tempPicture: String;
    pub rotation: i32;
    pub center: i32;
    pub subtype: i32;
    pub parentElement: i32;
    pub childType: i32;
    pub childData: i32;
    pub hidden: bool;
    pub topRow: i32;
    pub rowsPerPage: i32;
    pub totalRows: i32;
    pub customBitmapFunction: i32;
    pub customBitmapFunction2: i32;
    pub customBitmapFunction3: i32;

    pub UDSElement()
    {
      self.eventPicture = -1;
      self.bitmapSlot = -1;
      self.optiontempvar = new int[500];
      self.optiontempvarOn = new bool[500];
      self.color = Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      self.tempPicture = "";
      self.parentElement = -1;
      self.childType = -1;
    }
  }
}
