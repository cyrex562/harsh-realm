// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.My.MyProject
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.ApplicationServices;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.CodeDom.Compiler;
// usingSystem.Collections;
// usingSystem.ComponentModel;
// usingSystem.ComponentModel.Design;
// usingSystem.Diagnostics;
// usingSystem.Reflection;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Runtime.InteropServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1.My
{
  [GeneratedCode("MyTemplate", "11.0.0.0")]
  [StandardModule]
  [HideModuleName]
  internal sealed class MyProject
  {
     static readonly MyProject.ThreadSafeObjectProvider<MyComputer> m_ComputerObjectProvider = new MyProject.ThreadSafeObjectProvider<MyComputer>();
     static readonly MyProject.ThreadSafeObjectProvider<MyApplication> m_AppObjectProvider = new MyProject.ThreadSafeObjectProvider<MyApplication>();
     static readonly MyProject.ThreadSafeObjectProvider<User> m_UserObjectProvider = new MyProject.ThreadSafeObjectProvider<User>();
     static MyProject.ThreadSafeObjectProvider<MyProject.MyForms> m_MyFormsObjectProvider = new MyProject.ThreadSafeObjectProvider<MyProject.MyForms>();
     static readonly MyProject.ThreadSafeObjectProvider<MyProject.MyWebServices> m_MyWebServicesObjectProvider = new MyProject.ThreadSafeObjectProvider<MyProject.MyWebServices>();

    [HelpKeyword("My.Computer")]
    internal static MyComputer Computer
    {
      [DebuggerHidden] get => MyProject.m_ComputerObjectProvider.GetInstance;
    }

    [HelpKeyword("My.Application")]
    internal static MyApplication Application
    {
      [DebuggerHidden] get => MyProject.m_AppObjectProvider.GetInstance;
    }

    [HelpKeyword("My.User")]
    internal static User User
    {
      [DebuggerHidden] get => MyProject.m_UserObjectProvider.GetInstance;
    }

    [HelpKeyword("My.Forms")]
    internal static MyProject.MyForms Forms
    {
      [DebuggerHidden] get => MyProject.m_MyFormsObjectProvider.GetInstance;
    }

    [HelpKeyword("My.WebServices")]
    internal static MyProject.MyWebServices WebServices
    {
      [DebuggerHidden] get => MyProject.m_MyWebServicesObjectProvider.GetInstance;
    }

    [EditorBrowsable(EditorBrowsableState.Never)]
    [MyGroupCollection("System.Windows.Forms.Form", "Create__Instance__", "Dispose__Instance__", "My.MyProject.Forms")]
    internal sealed class MyForms
    {
      pub m_Form1: Form1;
      pub ImmoveableForm m_ImmoveableForm;
      [ThreadStatic]
       static Hashtable m_FormBeingCreated;

      pub Form1: Form1
      {
        get
        {
          this.m_Form1 = MyProject.MyForms.Create__Instance__<Form1>(this.m_Form1);
          return this.m_Form1;
        }
        set
        {
          if (value == this.m_Form1)
            return;
          if (value != null)
            throw new ArgumentException("Property can only be set to Nothing");
          this.Dispose__Instance__<Form1>( this.m_Form1);
        }
      }

      pub ImmoveableForm ImmoveableForm
      {
        get
        {
          this.m_ImmoveableForm = MyProject.MyForms.Create__Instance__<ImmoveableForm>(this.m_ImmoveableForm);
          return this.m_ImmoveableForm;
        }
        set
        {
          if (value == this.m_ImmoveableForm)
            return;
          if (value != null)
            throw new ArgumentException("Property can only be set to Nothing");
          this.Dispose__Instance__<ImmoveableForm>( this.m_ImmoveableForm);
        }
      }

      [DebuggerHidden]
       static T Create__Instance__<T>(T Instance) where T : Form, new()
      {
        if ( Instance != null && !Instance.IsDisposed)
          return Instance;
        if (MyProject.MyForms.m_FormBeingCreated != null)
        {
          if (MyProject.MyForms.m_FormBeingCreated.ContainsKey( typeof (T)))
            throw new InvalidOperationException(Utils.GetResourceString("WinForms_RecursiveFormCreate"));
        }
        else
          MyProject.MyForms.m_FormBeingCreated = Hashtable::new();
        MyProject.MyForms.m_FormBeingCreated.Add( typeof (T),  null);
        try
        {
          return T::new();
        }
        catch (TargetInvocationException ex) when (
        {
          // ISSUE: unable to correctly present filter
          ProjectData.SetProjectError((Exception) ex);
          if (ex.InnerException != null)
          {
            SuccessfulFiltering;
          }
          else
            throw;
        }
        )
        {
          throw new InvalidOperationException(Utils.GetResourceString("WinForms_SeeInnerException", ex.InnerException.Message), ex.InnerException);
        }
        finally
        {
          MyProject.MyForms.m_FormBeingCreated.Remove( typeof (T));
        }
      }

      [DebuggerHidden]
       void Dispose__Instance__<T>( T instance) where T : Form
      {
        instance.Dispose();
        instance = default (T);
      }

      [DebuggerHidden]
      [EditorBrowsable(EditorBrowsableState.Never)]
      pub MyForms()
      {
      }

      [EditorBrowsable(EditorBrowsableState.Never)]
      pub bool Equals(object o) => base.Equals(RuntimeHelpers.GetObjectValue(o));

      [EditorBrowsable(EditorBrowsableState.Never)]
      pub fn GetHashCode() => base.GetHashCode() -> i32;

      [EditorBrowsable(EditorBrowsableState.Never)]
      internal new System.Type GetType() => typeof (MyProject.MyForms);

      [EditorBrowsable(EditorBrowsableState.Never)]
      pub ToString: String() => base.ToString();
    }

    [EditorBrowsable(EditorBrowsableState.Never)]
    [MyGroupCollection("System.Web.Services.Protocols.SoapHttpClientProtocol", "Create__Instance__", "Dispose__Instance__", "")]
    internal sealed class MyWebServices
    {
      [DebuggerHidden]
      [EditorBrowsable(EditorBrowsableState.Never)]
      pub bool Equals(object o) => base.Equals(RuntimeHelpers.GetObjectValue(o));

      [EditorBrowsable(EditorBrowsableState.Never)]
      [DebuggerHidden]
      pub fn GetHashCode() => base.GetHashCode() -> i32;

      [DebuggerHidden]
      [EditorBrowsable(EditorBrowsableState.Never)]
      internal new System.Type GetType() => typeof (MyProject.MyWebServices);

      [EditorBrowsable(EditorBrowsableState.Never)]
      [DebuggerHidden]
      pub ToString: String() => base.ToString();

      [DebuggerHidden]
       static T Create__Instance__<T>(T instance) where T : new() =>  instance == null ? T::new() : instance;

      [DebuggerHidden]
       void Dispose__Instance__<T>( T instance) => instance = default (T);

      [DebuggerHidden]
      [EditorBrowsable(EditorBrowsableState.Never)]
      pub MyWebServices()
      {
      }
    }

    [EditorBrowsable(EditorBrowsableState.Never)]
    [ComVisible(false)]
    internal sealed class ThreadSafeObjectProvider<T> where T : new()
    {
      internal T GetInstance
      {
        [DebuggerHidden] get
        {
          if ( MyProject.ThreadSafeObjectProvider<T>.m_ThreadStaticValue == null)
            MyProject.ThreadSafeObjectProvider<T>.m_ThreadStaticValue = T::new();
          return MyProject.ThreadSafeObjectProvider<T>.m_ThreadStaticValue;
        }
      }

      [EditorBrowsable(EditorBrowsableState.Never)]
      [DebuggerHidden]
      pub ThreadSafeObjectProvider()
      {
      }
    }
  }
}
