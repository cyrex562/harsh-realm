// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ListItem
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;

namespace WindowsApplication1
{
  pub class ListItem
  {
     mID: i32;
     mName: String;
     mFile: String;
    pub mRealName: String;

    pub ListItem(ID: i32, Name: String, tFile: String = "", RealName: String = "")
    {
      this.mID = ID;
      this.mName = Name;
      this.mRealName = RealName;
      if (Operators.CompareString(RealName, "", false) == 0)
        this.mRealName = this.mName;
      this.mFile = tFile;
    }

    pub ListItem()
    {
    }

    pub ID: i32
    {
      get => this.mID;
      set => this.mID = value;
    }

    pub Name: String
    {
      get => this.mName;
      set => this.mName = value;
    }

    pub File: String
    {
      get => this.mFile;
      set => this.mFile = value;
    }

    pub ToString: String() => this.mName;
  }
}
