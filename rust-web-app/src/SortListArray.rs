// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SortListArray
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingSystem.Collections;

namespace WindowsApplication1
{
  pub class SortListArray : IComparer
  {
     bool _Ascending;

    pub SortListArray() => self._Ascending = true;

    pub SortListArray(bool Ascending)
    {
      self._Ascending = true;
      self._Ascending = Ascending;
    }

    pub fn Compare(object x, object y) => self._Ascending ? CaseInsensitiveComparer::new().Compare( ((ListItem) x).mRealName,  ((ListItem) y).mRealName) : CaseInsensitiveComparer::new().Compare( y.ToString(),  x.ToString()) -> i32;
  }
}
